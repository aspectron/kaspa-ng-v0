#![allow(dead_code)]

use crate::*;
use workflow_core::runtime::{self, Runtime};

pub struct WalletData {
    /// Data version
    version: u16,
    /// Wallet groups
    groups: Vec<WalletGroup>,
}

pub struct WalletGroup {
    /// Wallet group version
    version: u8,
    /// BIP39 mnemonic phrases: sequences of words representing cryptographic keys.
    encrypted_mnemonic: Vec<u8>,
    /// Wallets
    wallets: Vec<Wallet>,
}

pub struct Wallet {
    /// Wallet version
    version: u8,
    /// Wallet name
    name: String,
    /// Account index in derivation path
    index: u64,
    /// Required Signature
    required_signature: WalletSignatureCount,
    /// Wallet type, e.g. Gen0 = KDX/PWA, Gen1 = Core
    wallet_gen: WalletGeneration,
    /// Extended Public key to generate addresses
    xpub: Vec<u8>,
}

pub enum WalletSignatureCount {
    Single,
    Multisig(u16),
}

pub struct WalletStore {}

impl WalletStore {
    pub fn get_saved() -> Option<String> {
        let _runtime = Runtime::get();
        log_trace!("runtime:  {}", _runtime);
        log_trace!("is_nw:  {}", runtime::is_nw());
        log_trace!("is_node:  {}", runtime::is_node());
        log_trace!("is_web:  {}", runtime::is_web());
        log_trace!("is_native:  {}", runtime::is_native());
        log_trace!("is_solana:  {}", runtime::is_solana());
        log_trace!("is_wasm:  {}", runtime::is_wasm());
        None
    }
}
