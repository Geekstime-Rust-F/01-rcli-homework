use std::io::stdin;

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{
        generic_array::{
            arr,
            typenum::{UInt, UTerm, B0, B1},
            GenericArray,
        },
        Aead, KeyInit,
    },
    ChaCha20Poly1305,
};

type NonceArray = GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

pub struct TextEncryptAndDecrypt {
    nonce: NonceArray,
}

impl TextEncryptAndDecrypt {
    pub fn new() -> Self {
        TextEncryptAndDecrypt {
            // nonce: ChaCha20Poly1305::generate_nonce(&mut OsRng),
            nonce: arr![u8; 167, 90, 200, 225, 165, 100, 88, 228, 185, 158, 96, 93],
        }
    }
}

impl TextEncryptAndDecrypt {
    pub fn process_text_encrypt(self, key: &str) -> Result<()> {
        let key = key.as_bytes().into();
        let cipher = ChaCha20Poly1305::new(key);

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let buffer = buffer.trim();

        let ciphertext = match cipher.encrypt(&self.nonce, buffer.as_ref()) {
            Ok(ciphertext) => ciphertext,
            Err(e) => return Err(anyhow!(e)),
        };

        println!("ciphertext: {:?}", URL_SAFE_NO_PAD.encode(ciphertext));
        Ok(())
    }
}

impl TextEncryptAndDecrypt {
    pub fn process_text_decrypt(self, key: &str) -> Result<()> {
        let key = key.as_bytes().into();
        let cipher = ChaCha20Poly1305::new(key);

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let buffer = URL_SAFE_NO_PAD.decode(buffer.trim())?;

        let plaintext = match cipher.decrypt(&self.nonce, buffer.as_ref()) {
            Ok(plaintext) => plaintext,
            Err(e) => return Err(anyhow!(e)),
        };

        println!("plaintext: {:?}", &plaintext); // TODO print ascii instead of
        Ok(())
    }
}
