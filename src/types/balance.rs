use std::ops::Deref;

use bdk_wallet::Balance as BdkBalance;
use wasm_bindgen::prelude::wasm_bindgen;

use super::Amount;

/// Balance, differentiated into various categories.
#[wasm_bindgen]
pub struct Balance(BdkBalance);

#[wasm_bindgen]
impl Balance {
    /// All coinbase outputs not yet matured
    #[wasm_bindgen(getter)]
    pub fn immature(&self) -> Amount {
        self.0.immature.into()
    }

    /// Unconfirmed UTXOs generated by a wallet tx
    #[wasm_bindgen(getter)]
    pub fn trusted_pending(&self) -> Amount {
        self.0.trusted_pending.into()
    }

    /// Unconfirmed UTXOs received from an external wallet
    #[wasm_bindgen(getter)]
    pub fn untrusted_pending(&self) -> Amount {
        self.0.untrusted_pending.into()
    }

    /// Confirmed and immediately spendable balance
    #[wasm_bindgen(getter)]
    pub fn confirmed(&self) -> Amount {
        self.0.confirmed.into()
    }

    /// Get sum of trusted_pending and confirmed coins.
    ///
    /// This is the balance you can spend right now that shouldn't get cancelled via another party
    /// double spending it.
    #[wasm_bindgen(getter)]
    pub fn trusted_spendable(&self) -> Amount {
        self.0.trusted_spendable().into()
    }

    /// Get the whole balance visible to the wallet.
    #[wasm_bindgen(getter)]
    pub fn total(&self) -> Amount {
        self.0.total().into()
    }
}

impl Deref for Balance {
    type Target = BdkBalance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkBalance> for Balance {
    fn from(inner: BdkBalance) -> Self {
        Balance(inner)
    }
}
