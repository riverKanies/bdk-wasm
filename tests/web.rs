//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use anyhow::{anyhow, Error};
use std::str::FromStr;
use web_sys::console;

use bdk_wallet::{
    bip39::Mnemonic,
    bitcoin::{bip32::DerivationPath, key::Secp256k1, AddressType, Network},
    descriptor,
    descriptor::IntoWalletDescriptor,
};
use bdk_wasm::{types::KeychainKind, WalletWrapper};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn new_test_wallet() -> Result<WalletWrapper, String> {
    let mnemonic_str = "drip drum plug universe beyond gasp cram action hurt keep awake tortoise luggage return luxury net jar awake mimic hurry critic curtain quiz kit";
    let esplora_url = "https://blockstream.info/testnet/api";

    let mnemonic = Mnemonic::from_str(mnemonic_str).unwrap();
    let (descriptor, change_descriptor) =
        mnemonic_to_descriptor(mnemonic, Network::Testnet, AddressType::P2wpkh)
            .expect("descriptor");

    console::log_1(&format!("descriptor: {}", descriptor).into());
    console::log_1(&format!("change_descriptor: {}", change_descriptor).into());

    WalletWrapper::new(
        "testnet".into(),
        descriptor,
        change_descriptor,
        esplora_url.to_string(),
    )
}

#[wasm_bindgen_test]
async fn test_wallet() {
    let stop_gap = 5;
    let parallel_requests = 1;

    let wallet = new_test_wallet().expect("wallet");
    wallet
        .full_scan(stop_gap, parallel_requests)
        .await
        .expect("full_scan");

    let address0 = wallet.peek_address(KeychainKind::External, 0);
    assert_eq!(
        address0.address(),
        "tb1q8vl3qjdxnm54psxn5vgzdf402ky23r0jjfd8cj".to_string()
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
        .full_scan(stop_gap, parallel_requests)
        .await
        .expect("second full_scan");

    let unused_addresses = wallet.list_unused_addresses(KeychainKind::External);
    assert_eq!(unused_addresses.len(), 2);

    console::log_1(&format!("unused_addresses: {:?}", unused_addresses).into());
}

pub fn mnemonic_to_descriptor(
    mnemonic: Mnemonic,
    network: Network,
    address_type: AddressType,
) -> Result<(String, String), Error> {
    let mnemonic_with_passphrase = (mnemonic, None);
    let secp = Secp256k1::new();

    let purpose = match address_type {
        AddressType::P2wpkh => "84h",
        AddressType::P2tr => "86h",
        AddressType::P2sh => "49h",
        _ => "44h",
    };

    let coin_type: &str = match network {
        Network::Bitcoin => "0h",
        _ => "1h",
    };

    let external_path = DerivationPath::from_str(&format!("m/{}/{}/0h/0", purpose, coin_type))?;
    let internal_path = DerivationPath::from_str(&format!("m/{}/{}/0h/1", purpose, coin_type))?;

    let (external_descriptor, internal_descriptor) = match address_type {
        AddressType::P2pkh => {
            let ext = descriptor!(pk((mnemonic_with_passphrase.clone(), external_path)))?;
            let int = descriptor!(pk((mnemonic_with_passphrase, internal_path)))?;
            (ext, int)
        }
        AddressType::P2sh => {
            let ext = descriptor!(sh(wpkh((mnemonic_with_passphrase.clone(), external_path))))?;
            let int = descriptor!(sh(wpkh((mnemonic_with_passphrase, internal_path))))?;
            (ext, int)
        }
        AddressType::P2wpkh => {
            let ext = descriptor!(wpkh((mnemonic_with_passphrase.clone(), external_path)))?;
            let int = descriptor!(wpkh((mnemonic_with_passphrase, internal_path)))?;
            (ext, int)
        }
        AddressType::P2tr => {
            let ext = descriptor!(tr((mnemonic_with_passphrase.clone(), external_path)))?;
            let int = descriptor!(tr((mnemonic_with_passphrase, internal_path)))?;
            (ext, int)
        }
        _ => {
            return Err(anyhow!("Unsupported address type"));
        }
    };

    let (external_wallet_descriptor, _) =
        external_descriptor.into_wallet_descriptor(&secp, network)?;
    let (internal_wallet_descriptor, _) =
        internal_descriptor.into_wallet_descriptor(&secp, network)?;

    external_wallet_descriptor.sanity_check()?;
    internal_wallet_descriptor.sanity_check()?;

    Ok((
        external_wallet_descriptor.to_string(),
        internal_wallet_descriptor.to_string(),
    ))
}
