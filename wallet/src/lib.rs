pub use addresses::{Address, AddressError, Prefix as AddressPrefix};
use kaspa_bip32::*;
pub use std::sync::Arc;
pub use wasm_bindgen::prelude::*;
pub use workflow_core::task::yield_executor;
pub use workflow_log::log_trace;

mod manager;
mod wallets;
mod wrapper;
mod storage;

pub enum WalletGeneration {
    Gen0,
    Gen1,
}
pub use storage::WalletStore;
pub use manager::WalletManager;
pub use wallets::*;
pub use wrapper::WalletWrapper;
pub fn dummy_address() -> Address {
    Address {
        prefix: AddressPrefix::Mainnet,
        payload: vec![0u8; 32],
        version: 0u8,
    }
}

#[cfg(any(test, feature = "test"))]
mod tests;
