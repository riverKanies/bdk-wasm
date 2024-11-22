mod descriptor;
mod esplora_wallet;

pub use descriptor::*;
pub use esplora_wallet::EsploraWallet;

#[cfg(feature = "metamask")]
mod esplora_mm_wallet;
#[cfg(feature = "bitcoind_rpc")]
mod rpc_wallet;
#[cfg(feature = "metamask")]
mod storage;

#[cfg(feature = "metamask")]
pub use esplora_mm_wallet::EsploraMMWallet;

#[cfg(feature = "bitcoind_rpc")]
pub use rpc_wallet::RpcWallet;
