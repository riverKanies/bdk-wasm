use std::str::FromStr;

use anyhow::{anyhow, Error};
use bdk_wallet::{
    bip39::Mnemonic,
    template::{
        Bip44, Bip44Public, Bip49, Bip49Public, Bip84, Bip84Public, Bip86, Bip86Public,
        DescriptorTemplate,
    },
    KeychainKind,
};
use bitcoin::bip32::{Fingerprint, Xpriv, Xpub};
use web_sys::console;

use crate::types::{AddressType, DescriptorPair, Network};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mnemonic_to_descriptor(
    mnemonic: &str,
    passphrase: &str,
    network: Network,
    address_type: AddressType,
) -> Result<DescriptorPair, JsValue> {
    let mnemonic = Mnemonic::parse(mnemonic).map_err(|e| JsValue::from(format!("{:?}", e)))?;
    let seed = mnemonic.to_seed(passphrase);
    let xprv = Xpriv::new_master(network, &seed).map_err(|e| JsValue::from(format!("{:?}", e)))?;

    let (internal, external) = _xpriv_to_descriptor(xprv, network, address_type)
        .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(DescriptorPair::new(external, internal))
}

#[wasm_bindgen]
pub fn xpriv_to_descriptor(
    extended_privkey: &str,
    network: Network,
    address_type: AddressType,
) -> Result<DescriptorPair, JsValue> {
    console::log_1(&format!("xpriv: {}", extended_privkey).into());

    let xprv = Xpriv::from_str(extended_privkey).map_err(|e| JsValue::from(format!("{:?}", e)))?;

    let (internal, external) = _xpriv_to_descriptor(xprv, network, address_type)
        .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(DescriptorPair::new(external, internal))
}

#[wasm_bindgen]
pub fn xpub_to_descriptor(
    extended_pubkey: &str,
    fingerprint: &str,
    network: Network,
    address_type: AddressType,
) -> Result<DescriptorPair, JsValue> {
    console::log_1(&format!("xpriv: {}", extended_pubkey).into());

    let xpub = Xpub::from_str(extended_pubkey).map_err(|e| JsValue::from(format!("{:?}", e)))?;
    let fingerprint =
        Fingerprint::from_hex(fingerprint).map_err(|e| JsValue::from(format!("{:?}", e)))?;

    let (internal, external) = _xpub_to_descriptor(xpub, fingerprint, network, address_type)
        .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(DescriptorPair::new(external, internal))
}

fn _xpriv_to_descriptor(
    xprv: Xpriv,
    network: Network,
    address_type: AddressType,
) -> Result<(String, String), Error> {
    let (ext, int) = match address_type {
        AddressType::P2pkh => (
            Bip44(xprv, KeychainKind::External).build(network.into())?,
            Bip44(xprv, KeychainKind::Internal).build(network.into())?,
        ),
        AddressType::P2sh => (
            Bip49(xprv, KeychainKind::External).build(network.into())?,
            Bip49(xprv, KeychainKind::Internal).build(network.into())?,
        ),
        AddressType::P2wpkh => (
            Bip84(xprv, KeychainKind::External).build(network.into())?,
            Bip84(xprv, KeychainKind::Internal).build(network.into())?,
        ),
        AddressType::P2tr => (
            Bip86(xprv, KeychainKind::External).build(network.into())?,
            Bip86(xprv, KeychainKind::Internal).build(network.into())?,
        ),
        _ => {
            return Err(anyhow!("Unsupported address type"));
        }
    };

    ext.0.sanity_check()?;
    int.0.sanity_check()?;

    Ok((ext.0.to_string(), int.0.to_string()))
}

fn _xpub_to_descriptor(
    xpub: Xpub,
    fingerprint: Fingerprint,
    network: Network,
    address_type: AddressType,
) -> Result<(String, String), Error> {
    let (ext, int) = match address_type {
        AddressType::P2pkh => (
            Bip44Public(xpub, fingerprint, KeychainKind::External).build(network.into())?,
            Bip44Public(xpub, fingerprint, KeychainKind::Internal).build(network.into())?,
        ),
        AddressType::P2sh => (
            Bip49Public(xpub, fingerprint, KeychainKind::External).build(network.into())?,
            Bip49Public(xpub, fingerprint, KeychainKind::Internal).build(network.into())?,
        ),
        AddressType::P2wpkh => (
            Bip84Public(xpub, fingerprint, KeychainKind::External).build(network.into())?,
            Bip84Public(xpub, fingerprint, KeychainKind::Internal).build(network.into())?,
        ),
        AddressType::P2tr => (
            Bip86Public(xpub, fingerprint, KeychainKind::External).build(network.into())?,
            Bip86Public(xpub, fingerprint, KeychainKind::Internal).build(network.into())?,
        ),
        _ => {
            return Err(anyhow!("Unsupported address type"));
        }
    };

    ext.0.sanity_check()?;
    int.0.sanity_check()?;

    Ok((ext.0.to_string(), int.0.to_string()))
}
