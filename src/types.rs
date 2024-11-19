use bdk_wallet::{AddressInfo as BdkAddressInfo, KeychainKind as BdkKeychainKind};
use wasm_bindgen::prelude::*; // Import the bdk type

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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Types of keychains
pub enum KeychainKind {
    /// External keychain, used for deriving recipient addresses.
    External,
    /// Internal keychain, used for deriving change addresses.
    Internal,
}

impl From<BdkKeychainKind> for KeychainKind {
    fn from(keychain_kind: BdkKeychainKind) -> Self {
        match keychain_kind {
            BdkKeychainKind::External => KeychainKind::External,
            BdkKeychainKind::Internal => KeychainKind::Internal,
        }
    }
}

impl From<KeychainKind> for BdkKeychainKind {
    fn from(keychain_kind: KeychainKind) -> Self {
        match keychain_kind {
            KeychainKind::External => BdkKeychainKind::External,
            KeychainKind::Internal => BdkKeychainKind::Internal,
        }
    }
}
