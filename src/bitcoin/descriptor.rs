use anyhow::{anyhow, Error};
use bdk_wallet::{
    bip39::Mnemonic,
    template::{
        Bip44, Bip44Public, Bip49, Bip49Public, Bip84, Bip84Public, Bip86, Bip86Public,
        DescriptorTemplate, DescriptorTemplateOut,
    },
    KeychainKind,
};
use bitcoin::{
    bip32::{Fingerprint, Xpriv, Xpub},
    AddressType, Network,
};

pub fn mnemonic_to_descriptor(
    mnemonic: &str,
    passphrase: &str,
    network: Network,
    address_type: AddressType,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error> {
    let xprv = mnemonic_to_xpriv(mnemonic, passphrase, network)?;

    xpriv_to_descriptor(xprv, network, address_type)
}

pub fn xpriv_to_descriptor(
    xprv: Xpriv,
    network: Network,
    address_type: AddressType,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error> {
    match address_type {
        AddressType::P2pkh => build_priv_descriptor(Bip44, xprv, network),
        AddressType::P2sh => build_priv_descriptor(Bip49, xprv, network),
        AddressType::P2wpkh => build_priv_descriptor(Bip84, xprv, network),
        AddressType::P2tr => build_priv_descriptor(Bip86, xprv, network),
        _ => Err(anyhow!("Unsupported address type")),
    }
}

pub fn xpub_to_descriptor(
    xpub: Xpub,
    fingerprint: Fingerprint,
    network: Network,
    address_type: AddressType,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error> {
    match address_type {
        AddressType::P2pkh => build_pub_descriptor(Bip44Public, xpub, fingerprint, network),
        AddressType::P2sh => build_pub_descriptor(Bip49Public, xpub, fingerprint, network),
        AddressType::P2wpkh => build_pub_descriptor(Bip84Public, xpub, fingerprint, network),
        AddressType::P2tr => build_pub_descriptor(Bip86Public, xpub, fingerprint, network),
        _ => Err(anyhow!("Unsupported address type")),
    }
}

pub fn mnemonic_to_xpriv(
    mnemonic: &str,
    passphrase: &str,
    network: Network,
) -> Result<Xpriv, Error> {
    let mnemonic = Mnemonic::parse(mnemonic)?;
    let xprv = Xpriv::new_master(network, &mnemonic.to_seed(passphrase))?;

    Ok(xprv)
}

fn build_priv_descriptor<T>(
    constructor: impl Fn(Xpriv, KeychainKind) -> T,
    xprv: Xpriv,
    network: Network,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error>
where
    T: DescriptorTemplate,
{
    let ext_template = constructor(xprv, KeychainKind::External);
    let int_template = constructor(xprv, KeychainKind::Internal);

    let ext = ext_template.build(network)?;
    let int = int_template.build(network)?;

    ext.0.sanity_check()?;
    int.0.sanity_check()?;

    Ok((ext, int))
}

fn build_pub_descriptor<T>(
    constructor: impl Fn(Xpub, Fingerprint, KeychainKind) -> T,
    xprv: Xpub,
    fingerprint: Fingerprint,
    network: Network,
) -> Result<(DescriptorTemplateOut, DescriptorTemplateOut), Error>
where
    T: DescriptorTemplate,
{
    let ext_template = constructor(xprv, fingerprint, KeychainKind::External);
    let int_template = constructor(xprv, fingerprint, KeychainKind::Internal);

    let ext = ext_template.build(network)?;
    let int = int_template.build(network)?;

    ext.0.sanity_check()?;
    int.0.sanity_check()?;

    Ok((ext, int))
}
