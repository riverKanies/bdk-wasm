use std::{cell::RefCell, rc::Rc};

use bdk_bitcoind_rpc::bitcoincore_rpc::{Auth, Client, RpcApi};
use bdk_wallet::Wallet;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::types::Network;

#[wasm_bindgen]
pub struct BitcoinRpcWallet {
    wallet: Rc<RefCell<Wallet>>,
    client: Rc<RefCell<Client>>,
}

#[wasm_bindgen]
impl BitcoinRpcWallet {
    #[wasm_bindgen(constructor)]
    pub fn new(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: String,
        rpc_user: String,
        rpc_pass: String,
    ) -> Result<BitcoinRpcWallet, String> {
        let wallet: Wallet = Wallet::create(external_descriptor, internal_descriptor)
            .network(network.into())
            .create_wallet_no_persist()
            .map_err(|e| format!("{:?}", e))?;

        let auth = Auth::UserPass(rpc_user.clone(), rpc_pass.clone());
        let client = Client::new(&url, auth).map_err(|e| format!("{:?}", e))?;

        Ok(BitcoinRpcWallet {
            wallet: Rc::new(RefCell::new(wallet)),
            client: Rc::new(RefCell::new(client)),
        })
    }

    #[wasm_bindgen]
    pub fn balance(&self) -> u64 {
        let balance = self.wallet.borrow().balance();
        balance.total().to_sat()
    }

    #[wasm_bindgen]
    pub fn get_blockchain_info(&self) -> Result<JsValue, String> {
        let block = self
            .client
            .borrow()
            .get_blockchain_info()
            .map_err(|e| format!("{:?}", e))?;

        let block_js = to_value(&block).map_err(|e| format!("{:?}", e))?;

        Ok(block_js)
    }
}
