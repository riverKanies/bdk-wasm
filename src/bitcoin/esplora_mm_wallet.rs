use std::str::FromStr;

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{PersistedWallet, Wallet};
use bitcoin::BlockHash;
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::bitcoin::storage::SnapWalletPersister;
use crate::types::{AddressInfo, KeychainKind, Network};

const STORAGE_KEY: &str = "btc_wallet";

#[wasm_bindgen]
pub struct EsploraMMWallet {
    wallet: PersistedWallet<SnapWalletPersister>,
    client: AsyncClient,
    persister: SnapWalletPersister,
}

#[wasm_bindgen]
impl EsploraMMWallet {
    pub async fn new(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: String,
    ) -> Result<EsploraMMWallet, String> {
        let mut persister = SnapWalletPersister::new(STORAGE_KEY.to_string());

        let wallet_opt = Wallet::load()
            .descriptor(
                KeychainKind::External.into(),
                Some(external_descriptor.clone()),
            )
            .descriptor(
                KeychainKind::Internal.into(),
                Some(internal_descriptor.clone()),
            )
            .extract_keys()
            .check_network(network.into())
            .load_wallet_async(&mut persister)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => Wallet::create(external_descriptor, internal_descriptor)
                .network(network.into())
                .create_wallet_async(&mut persister)
                .await
                .map_err(|e| format!("{:?}", e))?,
        };

        let client = Builder::new(&url)
            .build_async()
            .map_err(|e| format!("{:?}", e))?;

        Ok(EsploraMMWallet {
            wallet,
            client,
            persister,
        })
    }

    pub async fn full_scan(
        &mut self,
        stop_gap: usize,
        parallel_requests: usize,
    ) -> Result<(), String> {
        let request = self.wallet.start_full_scan();
        let update = self
            .client
            .full_scan(request, stop_gap, parallel_requests)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet
            .apply_update_at(update, Some(now))
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub async fn sync(&mut self, parallel_requests: usize) -> Result<(), String> {
        let request = self.wallet.start_sync_with_revealed_spks();
        let update = self
            .client
            .sync(request, parallel_requests)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet
            .apply_update_at(update, Some(now))
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub fn balance(&self) -> u64 {
        let balance = self.wallet.balance();
        balance.total().to_sat()
    }

    pub fn next_unused_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.wallet.next_unused_address(keychain.into()).into()
    }

    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.wallet.peek_address(keychain.into(), index).into()
    }

    pub fn reveal_next_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.wallet.reveal_next_address(keychain.into()).into()
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.wallet
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    pub fn take_staged(&mut self) -> JsValue {
        let changeset_opt = self.wallet.take_staged();

        match changeset_opt {
            Some(changeset) => {
                let changeset_js = to_value(&changeset)
                    .map_err(|e| format!("{:?}", e))
                    .expect("should not fail to serialize changeset");
                changeset_js
            }
            None => JsValue::null(),
        }
    }

    pub async fn persist(&mut self) -> Result<bool, JsValue> {
        self.wallet
            .persist_async(&mut self.persister)
            .await
            .map_err(|e| JsValue::from(format!("{:?}", e)))
    }

    pub async fn get_block_by_hash(&self, block_hash: String) -> Result<JsValue, String> {
        let block_hash =
            BlockHash::from_str(block_hash.as_str()).map_err(|e| format!("{:?}", e))?;

        let block = self
            .client
            .get_block_by_hash(&block_hash)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let block_js = to_value(&block).map_err(|e| format!("{:?}", e))?;

        Ok(block_js)
    }
}
