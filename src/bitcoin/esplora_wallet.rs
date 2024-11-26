use std::str::FromStr;

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{descriptor::IntoWalletDescriptor, Wallet};
use bitcoin::{
    bip32::{Fingerprint, Xpriv, Xpub},
    BlockHash,
};
use js_sys::Date;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{
    bitcoin::{mnemonic_to_descriptor, xpriv_to_descriptor, xpub_to_descriptor},
    result::JsResult,
    types::{AddressInfo, AddressType, Balance, KeychainKind, Network},
};

#[wasm_bindgen]
pub struct EsploraWallet {
    wallet: Wallet,
    client: AsyncClient,
}

#[wasm_bindgen]
impl EsploraWallet {
    fn create<D>(
        network: Network,
        external_descriptor: D,
        internal_descriptor: D,
        url: &str,
    ) -> Result<EsploraWallet, anyhow::Error>
    where
        D: IntoWalletDescriptor + Send + Clone + 'static,
    {
        let wallet = Wallet::create(external_descriptor, internal_descriptor)
            .network(network.into())
            .create_wallet_no_persist()?;

        let client = Builder::new(&url).build_async()?;

        Ok(EsploraWallet { wallet, client })
    }

    pub fn from_descriptors(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: &str,
    ) -> JsResult<EsploraWallet> {
        Self::create(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn from_mnemonic(
        mnemonic: &str,
        passphrase: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> JsResult<EsploraWallet> {
        let (external_descriptor, internal_descriptor) =
            mnemonic_to_descriptor(&mnemonic, &passphrase, network.into(), address_type.into())
                .map_err(|e| JsError::new(&e.to_string()))?;

        Self::create(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn from_xpriv(
        extended_privkey: &str,
        fingerprint: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> JsResult<EsploraWallet> {
        let xprv = Xpriv::from_str(extended_privkey).map_err(|e| JsError::new(&e.to_string()))?;
        let fingerprint = Fingerprint::from_hex(fingerprint)?;

        let (external_descriptor, internal_descriptor) =
            xpriv_to_descriptor(xprv, fingerprint, network.into(), address_type.into())
                .map_err(|e| JsError::new(&e.to_string()))?;

        Self::create(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn from_xpub(
        extended_pubkey: &str,
        fingerprint: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> JsResult<EsploraWallet> {
        let xpub = Xpub::from_str(extended_pubkey)?;
        let fingerprint = Fingerprint::from_hex(fingerprint)?;

        let (external_descriptor, internal_descriptor) =
            xpub_to_descriptor(xpub, fingerprint, network.into(), address_type.into())
                .map_err(|e| JsError::new(&e.to_string()))?;

        Self::create(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    pub fn load(changeset: JsValue, url: &str) -> JsResult<EsploraWallet> {
        let changeset = from_value(changeset)?;
        let wallet_opt = Wallet::load().load_wallet_no_persist(changeset)?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => return Err(JsError::new("Failed to load wallet, check the changeset")),
        };

        let client = Builder::new(&url).build_async()?;

        Ok(EsploraWallet { wallet, client })
    }

    pub async fn full_scan(&mut self, stop_gap: usize, parallel_requests: usize) -> JsResult<()> {
        let request = self.wallet.start_full_scan();
        let update = self
            .client
            .full_scan(request, stop_gap, parallel_requests)
            .await?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet.apply_update_at(update, now)?;

        Ok(())
    }

    pub async fn sync(&mut self, parallel_requests: usize) -> JsResult<()> {
        let request = self.wallet.start_sync_with_revealed_spks();
        let update = self.client.sync(request, parallel_requests).await?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet.apply_update_at(update, now)?;

        Ok(())
    }

    pub fn network(&self) -> Network {
        self.wallet.network().into()
    }

    pub fn balance(&self) -> Balance {
        self.wallet.balance().into()
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

    pub fn reveal_addresses_to(&mut self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
        self.wallet
            .reveal_addresses_to(keychain.into(), index)
            .map(Into::into)
            .collect()
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.wallet
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    pub fn list_unspent(&self) -> JsResult<Vec<JsValue>> {
        self.wallet
            .list_unspent()
            .map(|output| to_value(&output).map_err(Into::into))
            .collect()
    }

    pub fn transactions(&self) -> JsResult<Vec<JsValue>> {
        self.wallet
            .transactions()
            .map(|tx| to_value(&tx.tx_node.tx).map_err(Into::into))
            .collect()
    }

    pub fn take_staged(&mut self) -> JsResult<JsValue> {
        let changeset_opt = self.wallet.take_staged();

        match changeset_opt {
            Some(changeset) => {
                let changeset_js = to_value(&changeset)?;
                Ok(changeset_js)
            }
            None => Ok(JsValue::null()),
        }
    }

    pub async fn get_block_by_hash(&self, block_hash: String) -> Result<JsValue, JsError> {
        let block_hash = BlockHash::from_str(block_hash.as_str())?;
        let block = self.client.get_block_by_hash(&block_hash).await?;
        let block_js = to_value(&block)?;

        Ok(block_js)
    }
}
