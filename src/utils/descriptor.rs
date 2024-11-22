use std::str::FromStr;

use bitcoin::bip32::{Fingerprint, Xpriv, Xpub};

use crate::types::{AddressType, DescriptorPair, Network};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mnemonic_to_descriptor(
    mnemonic: &str,
    passphrase: &str,
    network: Network,
    address_type: AddressType,
) -> Result<DescriptorPair, JsValue> {
    let (external, internal) = crate::bitcoin::mnemonic_to_descriptor(
        mnemonic,
        passphrase,
        network.into(),
        address_type.into(),
    )
    .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(DescriptorPair::new(
        external.0.to_string_with_secret(&external.1),
        internal.0.to_string_with_secret(&internal.1),
    ))
}

#[wasm_bindgen]
pub fn xpriv_to_descriptor(
    extended_privkey: &str,
    network: Network,
    address_type: AddressType,
) -> Result<DescriptorPair, JsValue> {
    let xprv = Xpriv::from_str(extended_privkey).map_err(|e| JsValue::from(format!("{:?}", e)))?;

    let (external, internal) =
        crate::bitcoin::xpriv_to_descriptor(xprv, network.into(), address_type.into())
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(DescriptorPair::new(
        external.0.to_string_with_secret(&external.1),
        internal.0.to_string_with_secret(&internal.1),
    ))
}

#[wasm_bindgen]
pub fn xpub_to_descriptor(
    extended_pubkey: &str,
    fingerprint: &str,
    network: Network,
    address_type: AddressType,
) -> Result<DescriptorPair, JsValue> {
    let xpub = Xpub::from_str(extended_pubkey).map_err(|e| JsValue::from(format!("{:?}", e)))?;
    let fingerprint =
        Fingerprint::from_hex(fingerprint).map_err(|e| JsValue::from(format!("{:?}", e)))?;

    let (external, internal) =
        crate::bitcoin::xpub_to_descriptor(xpub, fingerprint, network.into(), address_type.into())
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(DescriptorPair::new(
        external.0.to_string(),
        internal.0.to_string(),
    ))
}

#[wasm_bindgen]
pub fn mnemonic_to_xpriv(
    mnemonic: &str,
    passphrase: &str,
    network: Network,
) -> Result<String, JsValue> {
    let xprv = crate::bitcoin::mnemonic_to_xpriv(mnemonic, passphrase, network.into())
        .map_err(|e| JsValue::from(format!("{:?}", e)))?;

    Ok(xprv.to_string())
}
