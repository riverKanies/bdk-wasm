//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "bitcoind_rpc", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use web_sys::console;

use bdk_wasm::{
    bitcoin::RpcWallet,
    mnemonic_to_descriptor, set_panic_hook,
    types::{AddressType, Network},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const NETWORK: Network = Network::Testnet;
const MNEMONIC: &str = "drip drum plug universe beyond gasp cram action hurt keep awake tortoise luggage return luxury net jar awake mimic hurry critic curtain quiz kit";

#[wasm_bindgen_test]
async fn test_rpc_wallet() {
    set_panic_hook();

    let descriptors =
        mnemonic_to_descriptor(MNEMONIC, "", NETWORK, AddressType::P2wpkh).expect("descriptor");
    let rpc_url = "http://127.0.0.1:18443";

    let wallet = RpcWallet::new(
        NETWORK,
        descriptors.external(),
        descriptors.internal(),
        rpc_url.to_string(),
        "polaruser".to_string(),
        "polarpass".to_string(),
    )
    .expect("rpc_wallet");

    let info = wallet.get_blockchain_info().expect("get_blockchain_info");

    console::log_1(&info);
}
