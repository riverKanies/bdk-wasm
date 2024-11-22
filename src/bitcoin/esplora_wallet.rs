use std::str::FromStr;

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{
    descriptor::IntoWalletDescriptor, ChangeSet, KeychainKind as BdkKeychainKind, Wallet,
};
use bitcoin::{
    bip32::{Fingerprint, Xpriv, Xpub},
    BlockHash,
};
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    bitcoin::{mnemonic_to_descriptor, xpriv_to_descriptor, xpub_to_descriptor},
    types::{AddressInfo, AddressType, KeychainKind, Network},
};

#[wasm_bindgen]
pub struct EsploraWallet {
    wallet: Wallet,
    client: AsyncClient,
}

#[wasm_bindgen]
impl EsploraWallet {
    fn load<D>(
        network: Network,
        external_descriptor: D,
        internal_descriptor: D,
        url: &str,
    ) -> Result<EsploraWallet, anyhow::Error>
    where
        D: IntoWalletDescriptor + Send + Clone + 'static,
    {
        let wallet_opt = Wallet::load()
            .descriptor(BdkKeychainKind::External, Some(external_descriptor.clone()))
            .descriptor(BdkKeychainKind::Internal, Some(internal_descriptor.clone()))
            .extract_keys()
            .check_network(network.into())
            .load_wallet_no_persist(ChangeSet::default())?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => Wallet::create(external_descriptor, internal_descriptor)
                .network(network.into())
                .create_wallet_no_persist()?,
        };

        let client = Builder::new(&url).build_async()?;

        Ok(EsploraWallet { wallet, client })
    }

    pub fn from_descriptors(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: &str,
    ) -> Result<EsploraWallet, JsValue> {
        Self::load(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsValue::from(format!("{:?}", e)))
    }

    pub fn from_mnemonic(
        mnemonic: &str,
        passphrase: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> Result<EsploraWallet, JsValue> {
        let (external_descriptor, internal_descriptor) =
            mnemonic_to_descriptor(&mnemonic, &passphrase, network.into(), address_type.into())
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;

        Self::load(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsValue::from(format!("{:?}", e)))
    }

    pub fn from_xpriv(
        extended_privkey: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> Result<EsploraWallet, JsValue> {
        let xprv =
            Xpriv::from_str(extended_privkey).map_err(|e| JsValue::from(format!("{:?}", e)))?;

        let (external_descriptor, internal_descriptor) =
            xpriv_to_descriptor(xprv, network.into(), address_type.into())
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;

        Self::load(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsValue::from(format!("{:?}", e)))
    }

    pub fn from_xpub(
        extended_pubkey: &str,
        fingerprint: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> Result<EsploraWallet, JsValue> {
        let xpub =
            Xpub::from_str(extended_pubkey).map_err(|e| JsValue::from(format!("{:?}", e)))?;
        let fingerprint =
            Fingerprint::from_hex(fingerprint).map_err(|e| JsValue::from(format!("{:?}", e)))?;

        let (external_descriptor, internal_descriptor) =
            xpub_to_descriptor(xpub, fingerprint, network.into(), address_type.into())
                .map_err(|e| JsValue::from(format!("{:?}", e)))?;

        Self::load(network, external_descriptor, internal_descriptor, url)
            .map_err(|e| JsValue::from(format!("{:?}", e)))
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