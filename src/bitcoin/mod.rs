mod descriptor;
mod wallet;

pub use descriptor::*;
pub use wallet::*;

#[cfg(feature = "esplora")]
mod esplora_wallet;

#[cfg(feature = "metamask")]
mod metamask_wallet;

#[cfg(feature = "bitcoind")]
mod rpc_wallet;

#[cfg(feature = "esplora")]
pub use esplora_wallet::EsploraWallet;

#[cfg(feature = "metamask")]
pub use metamask_wallet::MetaMaskWallet;

#[cfg(feature = "bitcoind")]
pub use rpc_wallet::RpcWallet;
