//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use web_sys::console;

use bdk_wasm::{
    mnemonic_to_descriptor, set_panic_hook,
    types::{AddressType, Network},
    xpriv_to_descriptor, xpub_to_descriptor,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const NETWORK: Network = Network::Signet;
const ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
const MNEMONIC: &str = "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose";

#[wasm_bindgen_test]
async fn test_mnemonic_to_descriptor() {
    set_panic_hook();

    let descriptors = mnemonic_to_descriptor(MNEMONIC, "", NETWORK, ADDRESS_TYPE)
        .expect("mnemonic_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );

    console::log_1(&format!("descriptor: {}", descriptors.external()).into());
    console::log_1(&format!("change_descriptor: {}", descriptors.internal()).into());
}

#[wasm_bindgen_test]
async fn test_xpriv_to_descriptor() {
    set_panic_hook();
    let xpriv = "tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp";

    let descriptors =
        xpriv_to_descriptor(xpriv, NETWORK, ADDRESS_TYPE).expect("xpriv_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );

    console::log_1(&format!("descriptor: {}", descriptors.external()).into());
    console::log_1(&format!("change_descriptor: {}", descriptors.internal()).into());
}

#[wasm_bindgen_test]
async fn test_xpub_to_descriptor() {
    set_panic_hook();
    let xpub = "tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq";
    let fingerprint = "27f9035f";

    let descriptors =
        xpub_to_descriptor(xpub, fingerprint, NETWORK, ADDRESS_TYPE).expect("xpub_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );

    console::log_1(&format!("descriptor: {}", descriptors.external()).into());
    console::log_1(&format!("change_descriptor: {}", descriptors.internal()).into());
}
