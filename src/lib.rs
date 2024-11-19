pub mod types;
mod utils;

use std::{cell::RefCell, rc::Rc, str::FromStr};

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder as EsploraBuilder},
    EsploraAsyncExt,
};
use bdk_wallet::{bitcoin::Network, ChangeSet, KeychainKind as BdkKeychainKind, Wallet};
use bitcoin::BlockHash;
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use types::{AddressInfo, KeychainKind};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub struct WalletWrapper {
    wallet: Rc<RefCell<Wallet>>,
    client: Rc<RefCell<AsyncClient>>,
}

#[wasm_bindgen]
impl WalletWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        network: String,
        external_descriptor: String,
        internal_descriptor: String,
        esplora_url: String,
    ) -> Result<WalletWrapper, String> {
        let network = match network.as_str() {
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "testnet4" => Network::Testnet4,
            "signet" => Network::Signet,
            "regtest" => Network::Regtest,
            _ => return Err("Invalid network".into()),
        };

        let wallet_opt = Wallet::load()
            .descriptor(BdkKeychainKind::External, Some(external_descriptor.clone()))
            .descriptor(BdkKeychainKind::Internal, Some(internal_descriptor.clone()))
            .extract_keys()
            .check_network(network)
            .load_wallet_no_persist(ChangeSet::default())
            .map_err(|e| format!("{:?}", e))?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => Wallet::create(external_descriptor, internal_descriptor)
                .network(network)
                .create_wallet_no_persist()
                .map_err(|e| format!("{:?}", e))?,
        };

        let client = EsploraBuilder::new(&esplora_url)
            .build_async()
            .map_err(|e| format!("{:?}", e))?;

        Ok(WalletWrapper {
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

        let change_set = self.wallet.borrow_mut().take_staged();
        let change_set_js = to_value(&change_set).map_err(|e| format!("{:?}", e))?;

        // Log the JsValue to the console
        console::log_1(&change_set_js);

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

        let change_set = self.wallet.borrow_mut().take_staged();
        let change_set_js = to_value(&change_set).map_err(|e| format!("{:?}", e))?;

        // Log the JsValue to the console
        console::log_1(&change_set_js);

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
