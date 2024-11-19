use std::{cell::RefCell, rc::Rc, str::FromStr};

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::Wallet;
use bitcoin::BlockHash;
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::types::{AddressInfo, KeychainKind, Network};

#[wasm_bindgen]
pub struct BitcoinEsploraWallet {
    wallet: Rc<RefCell<Wallet>>,
    client: Rc<RefCell<AsyncClient>>,
}

#[wasm_bindgen]
impl BitcoinEsploraWallet {
    #[wasm_bindgen(constructor)]
    pub fn new(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: String,
    ) -> Result<BitcoinEsploraWallet, String> {
        let wallet = Wallet::create(external_descriptor, internal_descriptor)
            .network(network.into())
            .create_wallet_no_persist()
            .map_err(|e| format!("{:?}", e))?;

        let client = Builder::new(&url)
            .build_async()
            .map_err(|e| format!("{:?}", e))?;

        Ok(BitcoinEsploraWallet {
            wallet: Rc::new(RefCell::new(wallet)),
            client: Rc::new(RefCell::new(client)),
        })
    }

    #[wasm_bindgen]
    pub async fn full_scan(&self, stop_gap: usize, parallel_requests: usize) -> Result<(), String> {
        let request = self.wallet.borrow().start_full_scan();
        let update = self
            .client
            .borrow()
            .full_scan(request, stop_gap, parallel_requests)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet
            .borrow_mut()
            .apply_update_at(update, Some(now))
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    #[wasm_bindgen]
    pub async fn sync(&self, parallel_requests: usize) -> Result<(), String> {
        let request = self.wallet.borrow().start_sync_with_revealed_spks();
        let update = self
            .client
            .borrow()
            .sync(request, parallel_requests)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet
            .borrow_mut()
            .apply_update_at(update, Some(now))
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn balance(&self) -> u64 {
        let balance = self.wallet.borrow().balance();
        balance.total().to_sat()
    }

    #[wasm_bindgen]
    pub fn next_unused_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.wallet
            .borrow_mut()
            .next_unused_address(keychain.into())
            .into()
    }

    #[wasm_bindgen]
    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.wallet
            .borrow()
            .peek_address(keychain.into(), index)
            .into()
    }

    #[wasm_bindgen]
    pub fn reveal_next_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.wallet
            .borrow_mut()
            .reveal_next_address(keychain.into())
            .into()
    }

    #[wasm_bindgen]
    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.wallet
            .borrow()
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    #[wasm_bindgen]
    pub async fn get_block_by_hash(&self, block_hash: String) -> Result<JsValue, String> {
        let block_hash =
            BlockHash::from_str(block_hash.as_str()).map_err(|e| format!("{:?}", e))?;

        let block = self
            .client
            .borrow()
            .get_block_by_hash(&block_hash)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let block_js = to_value(&block).map_err(|e| format!("{:?}", e))?;

        Ok(block_js)
    }
}
