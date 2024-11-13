//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use bdk_wasm::WalletWrapper;
use wasm_bindgen_test::*;
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

fn test_wallet() -> Result<WalletWrapper, String> {
    let network = "signet".to_string();
    let descriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m"
    .to_string();
    let change_descriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr"
    .to_string();
    let esplora = "https://mutinynet.com/api".to_string();

    WalletWrapper::new(network, descriptor, change_descriptor, esplora)
}

#[wasm_bindgen_test]
async fn test_sync() {
    let wallet = test_wallet().expect("wallet");
    let promise = wallet.sync(2);
    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .expect("sync failed");
    
    console::log_1(&format!("Sync completed with result: {:?}", result).into());

    let balance = wallet.balance();
    console::log_1(&format!("Balance after sync: {}", balance).into());

    assert!(balance > 0);
}

#[wasm_bindgen_test]
async fn test_balance() {
    let wallet = test_wallet().expect("wallet");
    let balance = wallet.balance();
    assert_eq!(balance, 0);
}

#[wasm_bindgen_test]
async fn test_new_address() {
    let wallet = test_wallet().expect("wallet");
    let new_address = wallet.get_new_address();

    assert_eq!(
        new_address,
        "tb1pkar3gerekw8f9gef9vn9xz0qypytgacp9wa5saelpksdgct33qdqs257jl".to_string()
    );
}
