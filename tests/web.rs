//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use bdk_wasm::{
    bitcoin::EsploraWallet,
    mnemonic_to_descriptor, mnemonic_to_xpriv, set_panic_hook,
    types::{AddressType, KeychainKind, Network},
    xpriv_to_descriptor, xpub_to_descriptor,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 2;
const NETWORK: Network = Network::Signet;
const ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
const MNEMONIC: &str = "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose";

#[wasm_bindgen_test]
async fn test_mnemonic_to_xpriv() {
    set_panic_hook();

    let xprv = mnemonic_to_xpriv(MNEMONIC, "", NETWORK).expect("xpub_to_descriptor");

    assert_eq!(
        xprv,
        "tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU"
    );
}

#[wasm_bindgen_test]
async fn test_mnemonic_to_descriptor() {
    set_panic_hook();

    let descriptors = mnemonic_to_descriptor(MNEMONIC, "", NETWORK, ADDRESS_TYPE)
        .expect("mnemonic_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e"
    );
}

#[wasm_bindgen_test]
async fn test_xpriv_to_descriptor() {
    set_panic_hook();
    let xpriv = "tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp";

    let descriptors =
        xpriv_to_descriptor(xpriv, NETWORK, ADDRESS_TYPE).expect("xpriv_to_descriptor");

    assert_eq!(
        descriptors.external(),
        "wpkh(tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp/84'/1'/0'/0/*)#7l6l9tgm"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh(tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp/84'/1'/0'/1/*)#0tl7c7cr"
    );
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
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );
    assert_eq!(
        descriptors.internal(),
        "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
}

#[wasm_bindgen_test]
async fn test_esplora_wallet() {
    set_panic_hook();

    let esplora_url = match NETWORK {
        Network::Bitcoin => "https://blockstream.info/api",
        Network::Testnet => "https://blockstream.info/testnet/api",
        Network::Testnet4 => "https://blockstream.info/testnet/api",
        Network::Signet => "https://mutinynet.com/api",
        Network::Regtest => "https://localhost:3000",
    };

    let mut wallet = EsploraWallet::from_mnemonic(MNEMONIC, "", NETWORK, ADDRESS_TYPE, esplora_url)
        .expect("esplora_wallet");

    wallet
        .full_scan(STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");

    let address0 = wallet.peek_address(KeychainKind::External, 0);
    assert_eq!(
        address0.address(),
        "tb1qjtgffm20l9vu6a7gacxvpu2ej4kdcsgc26xfdz".to_string()
    );

    let balance = wallet.balance();
    assert_eq!(balance, 0);

    let address1 = wallet.next_unused_address(KeychainKind::External);
    assert_eq!(address1.keychain(), KeychainKind::External);
    assert_eq!(address1.index(), 0);

    let address2: bdk_wasm::types::AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    assert_eq!(address2.index(), 1);

    let address3 = wallet.next_unused_address(KeychainKind::External);
    assert_eq!(address3.index(), 0);

    // Should do a single call to the server (for each keychain)
    wallet.sync(1).await.expect("sync");

    // Should do a stop_gap calls to the server (for each keychain) and not start from beginning
    wallet
        .full_scan(STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("second full_scan");

    let unused_addresses = wallet.list_unused_addresses(KeychainKind::External);
    assert_eq!(unused_addresses.len(), 2);
}
