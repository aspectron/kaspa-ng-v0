use crate::WalletWrapper;
use std::sync::Arc;

pub struct WalletManager{

}

impl WalletManager{
    /// WalletManager Constructor
    pub fn new()->Self{
        Self {

        }
    }

    /// Open wallet from mnemonic
    pub fn open_wallet_from_mnemonic(
        &self,
        mnemonic: String,
        password: String
    )->Arc<dyn WalletWrapper>{

    }
}