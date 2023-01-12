
use bip32::{Prefix, ExtendedKeyAttrs, PrivateKey, Error, ExtendedPrivateKey, secp256k1::ecdsa::SigningKey};
use bip32::{KEY_SIZE, ChildNumber, PublicKey, ExtendedKey, ExtendedPublicKey, PrivateKeyBytes};
use hmac::Mac;
//use bip32::{Mnemonic, XPrv}
//use rand_core::OsRng;
use std::fmt::Debug;
use std::str::FromStr;
use zeroize::Zeroizing;
use zeroize::Zeroize;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use addresses::{Address, Prefix as AddressPrefix};

type Result<T> = core::result::Result<T, Error>;
type HmacSha512 = hmac::Hmac<sha2::Sha512>;

pub enum AddressType{
    Receive = 0,
    Change
}
impl ToString for AddressType{
    fn to_string(&self) -> String {
        match self {
            Self::Receive => "Receive",
            Self::Change => "Change"
        }.to_string()
    }
}

impl AddressType {
    fn index(&self) -> i8 {
        match self {
            Self::Receive => 0,
            Self::Change => 1
        }
    }
}

#[derive(Clone)]
pub struct XKey<K: PrivateKey+Clone> {
    /// Derived private key
    private_key: K,

    /// Extended key attributes.
    attrs: ExtendedKeyAttrs,
}

impl<K> XKey<K>
where
    K: PrivateKey + Clone,
{


    pub fn from_str(xpriv: &str)->Result<Self>{
        let xpriv_key = ExtendedPrivateKey::<K>::from_str(xpriv)?;
        //println!("xpriv_key: {}", xpriv_key.to_string(Prefix::XPRV).as_str());
        let mut attrs = xpriv_key.attrs().clone();
        attrs.depth = 0;
        println!("attrs: {:?}", attrs);
    
        let key =  Self{
            private_key: xpriv_key.private_key().clone(),
            attrs
        };
    
        Ok(key)
    }

    pub fn derive_address(&self, address_type: AddressType, index: i32)->Result<Address>{
        let address_path = format!("44'/972/0'/{}'/{}'", address_type.index(), index);
        let children = address_path.split("/");
        let mut key = self.clone();
        for child in children{
            let c = child.parse::<ChildNumber>()?;
            //key = key.derive_child(c.clone())?;
            //println!("\nc:    {c:?}");
            //println!("key: {:#?}", key2);
            /*
            println!("key: {:?}",
                //key.to_string(Prefix::XPRV).as_str(),
                key2.to_string(Prefix::XPRV).as_str()
            );
            */
            key = key.derive_child(c.clone())?;
        }

        let pubkey = &key.private_key.public_key().to_bytes()[1..];
        let address = Address{
            prefix: AddressPrefix::Mainnet,
            version: 0,
            payload: pubkey.to_vec()
        };

        //let address_str: String = address.into();

        Ok(address)
    }

    pub fn derive_child(&self, child_number: ChildNumber) -> Result<Self> {

        let digest = Ripemd160::digest(&Sha256::digest(&self.private_key.public_key().to_bytes()[1..]));
        let fingerprint = digest[..4].try_into().expect("digest truncated");

        /*
        println!("\n_deriveWithNumber {}, {}, {}, fingerprint:{}", 
            child_number,
            child_number.is_hardened(),
            hex::encode(self.private_key.public_key().to_bytes()),
            hex::encode(&fingerprint)
        );
        */

        let depth = self.attrs.depth.checked_add(1).ok_or(Error::Depth)?;

        let mut hmac =
            HmacSha512::new_from_slice(&self.attrs.chain_code).map_err(|_| Error::Crypto)?;

        if child_number.is_hardened() {
            hmac.update(&[0]);
            hmac.update(&self.private_key.to_bytes());
            //println!("data: {}{}", hex::encode(self.private_key.to_bytes()), hex::encode(child_number.to_bytes()));
        } else {
            hmac.update(&self.private_key.public_key().to_bytes()[1..]);
            //println!("data: {}{}", hex::encode(&self.private_key.public_key().to_bytes()[1..]), hex::encode(child_number.to_bytes()));
        }

        hmac.update(&child_number.to_bytes());

        let result = hmac.finalize().into_bytes();
        //println!("chainCode: {}", hex::encode(self.attrs.chain_code));
        //println!("hash: {}", hex::encode(&result));

        let (child_key, chain_code) = result.split_at(KEY_SIZE);

        // We should technically loop here if a `secret_key` is zero or overflows
        // the order of the underlying elliptic curve group, incrementing the
        // index, however per "Child key derivation (CKD) functions":
        // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#child-key-derivation-ckd-functions
        //
        // > "Note: this has probability lower than 1 in 2^127."
        //
        // ...so instead, we simply return an error if this were ever to happen,
        // as the chances of it happening are vanishingly small.
        let private_key = self.private_key.derive_child(child_key.try_into()?)?;

        

        let attrs = ExtendedKeyAttrs {
            parent_fingerprint: fingerprint,
            child_number,
            chain_code: chain_code.try_into()?,
            depth,
        };

        let derived = Self { private_key, attrs };

        /*
        derived.log(Prefix::XPRV);

        println!("index:{}\nderived.childIndex:{}\nderived.xprivkey:{}",
            child_number,
            derived.attrs.child_number,
            derived.to_string(Prefix::XPRV).as_str()
        );

        println!("==================\n");
        */

        Ok(derived)
    }


    /// Serialize the raw private key as a byte array.
    pub fn to_bytes(&self) -> PrivateKeyBytes {
        self.private_key.to_bytes()
    }

    /// Serialize this key as an [`ExtendedKey`].
    pub fn to_extended_key(&self, prefix: Prefix) -> ExtendedKey {
        // Add leading `0` byte
        let mut key_bytes = [0u8; KEY_SIZE + 1];
        key_bytes[1..].copy_from_slice(&self.to_bytes());

        ExtendedKey {
            prefix,
            attrs: self.attrs.clone(),
            key_bytes,
        }
    }

    /// Serialize this key as a self-[`Zeroizing`] `String`.
    pub fn to_string(&self, prefix: Prefix) -> Zeroizing<String> {
        let key = self.to_extended_key(prefix);
        
        //println!("str: {str}");
        Zeroizing::new(key.to_string())
    }

    pub fn log(&self, prefix: Prefix){
        let key = self.to_extended_key(prefix);
        let mut buf = [0u8; ExtendedKey::MAX_BASE58_SIZE];
        let _str = write_base58(&key, &mut buf)
            .map_err(|_| std::fmt::Error).unwrap();
    }

    /*
    pub fn public_key(&self) -> ExtendedPublicKey<K::PublicKey>{
        self.into()
    }
    */
    pub fn private_key(&self) -> &K{
        &self.private_key
    }
    pub fn attrs(&self) -> &ExtendedKeyAttrs{
        &self.attrs
    }
}

/*
impl<K> From<&XKey<K>> for ExtendedPublicKey<K::PublicKey>
where
    K: PrivateKey + Clone,
{
    fn from(xprv: &XKey<K>) -> ExtendedPublicKey<K::PublicKey> {
        ExtendedPublicKey {
            public_key: xprv.private_key().public_key(),
            attrs: xprv.attrs().clone(),
        }
    }
}
*/

/// Write a Base58-encoded key to the provided buffer, returning a `&str`
/// containing the serialized data.
///
/// Note that this type also impls [`Display`] and therefore you can
/// obtain an owned string by calling `to_string()`.
pub fn write_base58<'a>(key:&ExtendedKey, buffer: &'a mut [u8; ExtendedKey::MAX_BASE58_SIZE]) -> Result<&'a str> {
    let mut bytes = [0u8; ExtendedKey::BYTE_SIZE]; // with 4-byte checksum
    bytes[..4].copy_from_slice(&key.prefix.to_bytes());
    bytes[4] = key.attrs.depth;
    bytes[5..9].copy_from_slice(&key.attrs.parent_fingerprint);
    bytes[9..13].copy_from_slice(&key.attrs.child_number.to_bytes());
    bytes[13..45].copy_from_slice(&key.attrs.chain_code);
    bytes[45..78].copy_from_slice(&key.key_bytes);
    
    println!("<Buffer {}>", hex::encode(key.prefix.to_bytes()));
    println!("<Buffer {}>", hex::encode([key.attrs.depth]));
    println!("<Buffer {}>", hex::encode(key.attrs.parent_fingerprint));
    println!("<Buffer {}>", hex::encode(key.attrs.child_number.to_bytes()));
    println!("<Buffer {}>", hex::encode(key.attrs.chain_code));
    println!("<Buffer {}>", hex::encode(key.key_bytes));

    println!("write_base58:{}", hex::encode(bytes));

    let base58_len = bs58::encode(&bytes).with_check().into(buffer.as_mut())?;
    bytes.zeroize();

    std::str::from_utf8(&buffer[..base58_len]).map_err(|_| Error::Base58)
}

impl<K:PrivateKey+Clone> Debug for XKey<K>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XKey")
            .field("depth", &self.attrs.depth)
            .field("child_number", &self.attrs.child_number)
            .field("chain_code", &hex::encode(&self.attrs.chain_code))
            .field("private_key", &hex::encode(&self.to_bytes()))
            .field("parent_fingerprint", &self.attrs.parent_fingerprint)
            .finish()
    }
}

fn test()->Result<()>{

    // Generate random Mnemonic using the default language (English)
    //let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

    // Derive a BIP39 seed value using the given password
    //let seed = mnemonic.to_seed("password");

    // Derive the root `XPrv` from the `seed` value
    //let root_xprv = XPrv::new(&seed)?;
    //assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);

    // Derive a child `XPrv` using the provided BIP32 derivation path
    //let child_path = "m/0/2147483647'/1/2147483646'";
    //let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;

    // Get the `XPub` associated with `child_xprv`.
    //let child_xpub = child_xprv.public_key();

    //let xpriv = child_xprv.to_string(Prefix::XPRV);
    //let xpub = child_xpub.to_string(Prefix::XPUB);
    let xpriv_str = "xprvsadasdasd.......";//xpriv.as_str();
    let hd_wallet = XKey::<SigningKey>::from_str(xpriv_str)?;
    let xpriv = hd_wallet.to_string(Prefix::XPRV);
    //let xpub = hd_wallet.public_key().to_string(Prefix::XPUB);
    println!("xpriv: {}", xpriv.as_str());
    //println!("xpub : {}", xpub);

    let mut addresses : Vec<String>= Vec::new();
    for index in 0..10{
        let address = hd_wallet.derive_address(AddressType::Receive, index)?;
        addresses.push(address.into());
    }

    for (index, address)in addresses.iter().enumerate(){
        println!("#{index}: {}", address);
    }

    

    //println!("key: {:?}", key.to_string(Prefix::XPRV).as_str());
    //let priv_key = xpriv_key. (child_number)

    /*
    // Serialize `child_xprv` as a string with the `xprv` prefix.
    let child_xprv_str = child_xprv.to_string(Prefix::XPRV);
    assert!(child_xprv_str.starts_with("xprv"));

    // Serialize `child_xpub` as a string with the `xpub` prefix.
    let child_xpub_str = child_xpub.to_string(Prefix::XPUB);
    assert!(child_xpub_str.starts_with("xpub"));

    // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
    let signing_key = child_xprv.private_key();
    let verification_key = child_xpub.public_key();

    // Sign and verify an example message using the derived keys.
    use bip32::secp256k1::ecdsa::{
        signature::{Signer, Verifier},
        Signature
    };

    let example_msg = b"Hello, world!";
    let signature: Signature = signing_key.sign(example_msg);
    assert!(verification_key.verify(example_msg, &signature).is_ok());
    */

    println!("");

    Ok(())
}

fn main() {
    let result = test();
    println!("result: {:?}", result);
}
