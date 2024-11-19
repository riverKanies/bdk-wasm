use bdk_wallet::{AddressInfo as BdkAddressInfo, KeychainKind as BdkKeychainKind};
use wasm_bindgen::prelude::*;

use super::KeychainKind;

/// A derived address and the index it was found at.
#[wasm_bindgen]
#[derive(Debug)]
pub struct AddressInfo {
    /// Child index of this address
    index: u32,
    /// Address
    address: String,
    /// Type of keychain
    keychain: BdkKeychainKind,
}

#[wasm_bindgen]
impl AddressInfo {
    #[wasm_bindgen(getter)]
    pub fn index(&self) -> u32 {
        self.index
    }

    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.address.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn keychain(&self) -> KeychainKind {
        self.keychain.into()
    }
}

impl From<BdkAddressInfo> for AddressInfo {
    fn from(address_info: BdkAddressInfo) -> Self {
        AddressInfo {
            address: address_info.address.to_string(),
            index: address_info.index,
            keychain: address_info.keychain.into(),
        }
    }
}
