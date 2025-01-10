use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]

extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, {}!wasm", s));
}
#[wasm_bindgen]
pub fn add(num1: i32, num2: i32) {
    log(&format!("num1+num2 is {}", num1 + num2));
}

#[wasm_bindgen]
pub struct KeyAndEncryptData(String, String);

#[wasm_bindgen]
impl KeyAndEncryptData {
    pub fn get_key(&self) -> String {
        self.0.clone()
    }
    pub fn get_encrypt_data(&self) -> String {
        self.1.clone()
    }
}

// impl IntoWasmAbi for KeyAndEncryptData {
//     type Abi = ();

//     fn into_abi(self) -> Self::Abi {
//         ()
//     }
// }

#[wasm_bindgen]
pub fn aes_gcm_encrypt(data: &str) -> KeyAndEncryptData {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes()).unwrap();
    let encode = BASE64_STANDARD.encode(ciphertext.as_slice());
    log(&format!("encode: {}", encode));
    let key = BASE64_STANDARD.encode(key.as_slice());
    KeyAndEncryptData(encode, key)
}

#[cfg(test)]
mod test {
    use aes_gcm::{
        aead::{Aead, OsRng},
        AeadCore, Aes256Gcm, Error, Key, KeyInit,
    };
    use base64::{prelude::BASE64_STANDARD, Engine};

    #[test]
    fn test_aead() -> Result<(), Error> {
        // The encryption key can be generated randomly:
        let key = Aes256Gcm::generate_key(OsRng);

        // Transformed from a byte array:
        let key: &[u8; 32] = &[42; 32];
        let key: &Key<Aes256Gcm> = key.into();

        // Note that you can get byte array from slice using the `TryInto` trait:
        let key: &[u8] = &[42; 32];
        let key: [u8; 32] = key.try_into().unwrap();

        // Alternatively, the key can be transformed directly from a byte slice
        // (panicks on length mismatch):
        let key = Key::<Aes256Gcm>::from_slice(&key);

        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
        let encode = BASE64_STANDARD.encode(ciphertext.as_slice());
        println!("encode: {}", encode);
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
        assert_eq!(&plaintext, b"plaintext message");
        Ok(())
    }
}
