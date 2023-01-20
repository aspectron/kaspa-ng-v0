use kaspa_bip32::*;
pub use wasm_bindgen::prelude::*;
pub use workflow_core::task::yield_executor;
pub use workflow_log::log_trace;
pub use addresses::{Address, AddressError, Prefix as AddressPrefix};
pub use std::sync::Arc;

mod wallets;
mod manager;
mod wrapper;
pub enum WalletGeneration{
    Gen0,
    Gen1
}
pub use manager::WalletManager;
pub use wrapper::WalletWrapper;
pub use wallets::*;
pub fn dummy_address()->Address{
    Address { prefix: AddressPrefix::Mainnet, payload: vec![0u8; 32], version: 0u8 }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
#[cfg(feature="test")]
mod tests;

#[cfg(target_arch = "wasm32")]
#[cfg(feature="test")]
pub use crate::tests::*;

