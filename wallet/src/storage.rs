
pub struct WalletData{
    /// Data version
    version: u16,
    /// Wallet groups
    groups: Vec<WalletGroup>
}

pub struct WalletGroup{
    /// Wallet group version
    version: u8,
    /// BIP39 mnemonic phrases: sequences of words representing cryptographic keys.
    encrypted_mnemonic: Vec<u8>,
    /// Wallets
    wallets: Vec<Wallet>
}

pub struct Wallet{
    /// Wallet version
    version: u8,
    /// Wallet name
    name: String,
    /// Account index in derivation path
    index: u64,
    /// Required Signature 
    required_signature: WalletSignatureCount,
    /// Wallet type, e.g. KDX/Core
    wallet_type: WalletType,
    /// Extended Public key to generate addresses
    xpub: Vec<u8>
}

pub enum WalletSignatureCount{
    Single,
    Multisig(u16)
}

pub enum WalletType{
    KDX,
    Core
}
