use bdk_wallet::{
    bitcoin::AddressType as BdkAddressType, AddressInfo as BdkAddressInfo,
    KeychainKind as BdkKeychainKind,
};
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

/// The different types of addresses.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AddressType {
    /// Pay to pubkey hash.
    P2pkh,
    /// Pay to script hash.
    P2sh,
    /// Pay to witness pubkey hash.
    P2wpkh,
    /// Pay to taproot.
    P2tr,
}

impl From<BdkAddressType> for AddressType {
    fn from(address_type: BdkAddressType) -> Self {
        match address_type {
            BdkAddressType::P2pkh => AddressType::P2pkh,
            BdkAddressType::P2sh => AddressType::P2sh,
            BdkAddressType::P2wpkh => AddressType::P2wpkh,
            BdkAddressType::P2tr => AddressType::P2tr,
            _ => panic!("Unsupported address type"),
        }
    }
}

impl From<AddressType> for BdkAddressType {
    fn from(address_type: AddressType) -> Self {
        match address_type {
            AddressType::P2pkh => BdkAddressType::P2pkh,
            AddressType::P2sh => BdkAddressType::P2sh,
            AddressType::P2wpkh => BdkAddressType::P2wpkh,
            AddressType::P2tr => BdkAddressType::P2tr,
        }
    }
}
