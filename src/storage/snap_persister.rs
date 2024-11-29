use std::{collections::HashMap, future::Future, pin::Pin};

use crate::{types::SnapPersisterError, SendFuture};
use bdk_wallet::{chain::Merge, AsyncWalletPersister, ChangeSet};
use bitcoin::base64::{prelude::BASE64_STANDARD, Engine};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value, Serializer};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;

type SnapState = HashMap<String, String>;

pub struct SnapPersister {
    key: String,
    serializer: Serializer,
}

impl SnapPersister {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            serializer: Serializer::json_compatible(),
        }
    }

    async fn read_changeset(&self) -> Result<ChangeSet, SnapPersisterError> {
        let state = self.read_snap_state().await?;
        web_sys::console::log_2(&"state as HashMap".into(), &to_value(&state).unwrap());

        self.extract_changeset(&state)
    }

    async fn write_changeset(&self, new_changeset: &ChangeSet) -> Result<(), SnapPersisterError> {
        web_sys::console::log_2(&"new_changeset".into(), &to_value(new_changeset).unwrap());

        let mut state = self.read_snap_state().await?;
        let mut changeset = self.extract_changeset(&state)?;
        changeset.merge(new_changeset.clone());

        web_sys::console::log_2(
            &"merged changeset to save".into(),
            &to_value(new_changeset).unwrap(),
        );

        let state_bytes = rmp_serde::to_vec(&changeset).map_err(SnapPersisterError::EncodeMRP)?;
        let state_b64 = BASE64_STANDARD.encode(&state_bytes);

        web_sys::console::log_2(&"state_b64 to save".into(), &to_value(&state_b64).unwrap());

        state.insert(self.key.clone(), state_b64);

        web_sys::console::log_2(&"state to save".into(), &to_value(&state).unwrap());

        let args = RequestArguments {
            method: "snap_manageState".to_string(),
            params: RequestParams {
                operation: "update".to_string(),
                new_state: Some(state),
            },
        };

        web_sys::console::log_2(
            &"args to save".into(),
            &args.serialize(&self.serializer).unwrap(),
        );

        let promise = snap_request(&args.serialize(&self.serializer).unwrap());
        JsFuture::from(promise)
            .await
            .map_err(SnapPersisterError::WriteSnapState)?;

        Ok(())
    }

    async fn read_snap_state(&self) -> Result<SnapState, SnapPersisterError> {
        let args = RequestArguments {
            method: "snap_manageState".to_string(),
            params: RequestParams {
                operation: "get".to_string(),
                new_state: None,
            },
        };

        web_sys::console::log_2(&"args in read_snap_state".into(), &to_value(&args).unwrap());

        let promise = snap_request(&to_value(&args).unwrap());
        let state = JsFuture::from(promise)
            .await
            .map_err(SnapPersisterError::ReadSnapState)?;

        web_sys::console::log_2(&"state read".into(), &state);

        if state.is_undefined() || state.is_null() {
            web_sys::console::log_1(&"no state".into());
            Ok(SnapState::new())
        } else {
            from_value(state).map_err(SnapPersisterError::Deserialize)
        }
    }

    fn extract_changeset(&self, state: &SnapState) -> Result<ChangeSet, SnapPersisterError> {
        if let Some(state_b64) = state.get(&self.key) {
            web_sys::console::log_2(&"state_b64".into(), &to_value(&state_b64).unwrap());

            let state_bytes = BASE64_STANDARD
                .decode(state_b64)
                .map_err(SnapPersisterError::DecodeBase64)?;

            web_sys::console::log_2(&"state_bytes".into(), &to_value(&state_bytes).unwrap());

            rmp_serde::from_slice(&state_bytes).map_err(SnapPersisterError::DecodeMRP)
        } else {
            web_sys::console::log_1(&"empty wallet state".into());
            Ok(ChangeSet::default())
        }
    }
}

impl AsyncWalletPersister for SnapPersister {
    type Error = SnapPersisterError;

    fn initialize<'a>(
        persister: &'a mut Self,
    ) -> Pin<Box<dyn Future<Output = Result<ChangeSet, Self::Error>> + Send + 'a>>
    where
        Self: 'a,
    {
        let fut = async move { persister.read_changeset().await };
        let send_fut = SendFuture(fut);
        Box::pin(send_fut)
    }

    fn persist<'a>(
        persister: &'a mut Self,
        changeset: &'a ChangeSet,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>
    where
        Self: 'a,
    {
        let fut = async move { persister.write_changeset(changeset).await };
        let send_fut = SendFuture(fut);
        Box::pin(send_fut)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = snap, js_name = request)]
    fn snap_request(params: &JsValue) -> Promise;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestArguments {
    method: String,
    params: RequestParams,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestParams {
    operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_state: Option<SnapState>,
}
