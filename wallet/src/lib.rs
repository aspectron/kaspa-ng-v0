
use std::time::Duration;
use wasm_bindgen::prelude::*;
use workflow_log::log_trace;
use workflow_core::task::*;

//use bip32;

mod xprivate_key;
mod xpublic_key;
mod private_key;
mod public_key;
mod xkey;

mod address_type;
mod types;
mod error;
mod result;
mod hd_wallet;
mod attrs;
mod child_number;
mod prefix;

pub use xprivate_key::ExtendedPrivateKey;
pub use xpublic_key::ExtendedPublicKey;
pub use private_key::PrivateKey;
pub use public_key::PublicKey;
pub use xkey::ExtendedKey;
pub use attrs::ExtendedKeyAttrs;
pub use prefix::Prefix;
pub use child_number::ChildNumber;
pub use types::*;
pub use address_type::AddressType;
pub use hd_wallet::HDWallet;

pub trait SecretKey2PublicKey{
    fn get_public_key(&self)->secp256k1_ffi::PublicKey;
}

impl SecretKey2PublicKey for secp256k1_ffi::SecretKey{
    fn get_public_key(&self)->secp256k1_ffi::PublicKey {
        secp256k1_ffi::PublicKey::from_secret_key_global(self)
    }
}

#[wasm_bindgen]
extern "C" {
    //#[wasm_bindgen(js_name = yield_now)]
    //fn yield_now_impl()->js_sys::Promise;

    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(callback:js_sys::Function);
}

pub async fn yield_now(){
    let promise = js_sys::Promise::new(&mut |res, _|{
        request_animation_frame(res);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

pub async fn yield_now1(){
   sleep(Duration::from_secs(1)).await;
}

fn init_yield(){
    let _ = js_sys::Function::new_no_args("
        if (!this.requestAnimationFrame){
            if (this.setImmediate)
                this.requestAnimationFrame = cb=>setImmediate(cb)
            else
                this.requestAnimationFrame = cb=>setTimeout(cb, 0)
        }
    ")
    .call0(&JsValue::undefined());
}

async fn test()->Result<()>{
    init_yield();

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
    let xpriv_str = "xprv9s21ZrQH143K4DoTUWmhygbsRQjAn1amZFxKpKtDtxabRpm9buGS5bbU4GuYDp7FtdReX5VbdGgoWwS7RuyWkV6aqYQUW6cX5McxWE8MN57";//xpriv.as_str();
    let hd_wallet = HDWallet::from_str(xpriv_str).await?;
    let xpriv = hd_wallet.to_string();
    let xpub = hd_wallet.public_key().to_string();
    log_trace!("xpriv: {}", xpriv.as_str());
    log_trace!("xpub : {}", xpub);

    let mut receive_addresses : Vec<String>= Vec::new();
    let mut change_addresses : Vec<String>= Vec::new();
    for index in 0..5000{
        let address = hd_wallet.derive_receive_address(index).await?;
        receive_addresses.push(address.into());
        //yield_now().await;
        let address = hd_wallet.derive_change_address(index).await?;
        change_addresses.push(address.into());
        if index % 3 == 0{
            yield_now().await;
        }
        
        if index % 50 == 0{
            log_trace!("generating {}", index);
        }
        //sleep(Duration::from_secs(1)).await;
    }

    log_trace!("Receive addresses:");
    for (index, address)in receive_addresses.iter().enumerate(){
        log_trace!("#{index}: {}", address);
    }
    log_trace!("Change addresses:");
    for (index, address)in change_addresses.iter().enumerate(){
        log_trace!("#{index}: {}", address);
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

/*
#[async_std::main]
async fn main() {
    let result = test().await;
    println!("result: {:?}", result);
}
*/


#[wasm_bindgen]
pub async fn test_addresses(){

    //spawn(async move {
        log_trace!("testing addresses");
        let result = test().await;
        log_trace!("result: {:?}", result);
    //});
}

