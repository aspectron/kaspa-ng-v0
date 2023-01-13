use secp256k1_ffi::{Secp256k1, SignOnly, scalar::Scalar};
pub use secp256k1_ffi::SecretKey;
use crate::types::*;
use crate::Result;
use crate::public_key::PublicKey;

pub trait PrivateKey: Sized {
    /// Public key type which corresponds to this private key.
    type PublicKey: PublicKey;

    /// Initialize this key from bytes.
    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self>;

    /// Serialize this key as bytes.
    fn to_bytes(&self) -> PrivateKeyBytes;

    /// Derive a child key from a parent key and the a provided tweak value,
    /// i.e. where `other` is referred to as "I sub L" in BIP32 and sourced
    /// from the left half of the HMAC-SHA-512 output.
    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self>;

    /// Get the [`Self::PublicKey`] that corresponds to this private key.
    fn public_key(&self) -> Self::PublicKey;
}

impl PrivateKey for SecretKey {
    type PublicKey = secp256k1_ffi::PublicKey;

    fn from_bytes(bytes: &PrivateKeyBytes) -> Result<Self> {
        Ok(SecretKey::from_slice(bytes)?)
    }

    fn to_bytes(&self) -> PrivateKeyBytes {
        *self.as_ref()
    }

    fn derive_child(&self, other: PrivateKeyBytes) -> Result<Self> {
        let child = *self;
        let other = Scalar::from_be_bytes(other)?;
        let child = child.add_tweak(&other)?;
        //child.add_assign(&other)?;
        Ok(child)
    }

    fn public_key(&self) -> Self::PublicKey {
        let engine = Secp256k1::<SignOnly>::signing_only();
        secp256k1_ffi::PublicKey::from_secret_key(&engine, self)
    }
}