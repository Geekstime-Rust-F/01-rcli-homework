use std::io::stdin;

use anyhow::{Result, anyhow};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
  aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305};

pub fn process_text_encrypt(key: &str) -> Result<()> {
  let key = key.as_bytes().into();
  let cipher = ChaCha20Poly1305::new(key);
  let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

  let mut buffer = String::new();
  stdin().read_line(&mut buffer)?;
  
  let ciphertext = match cipher.encrypt(&nonce, buffer.as_ref()){
    Ok(ciphertext) => ciphertext,
    Err(e) => return Err(anyhow!(e)),
  };

  println!("ciphertext: {:?}", URL_SAFE_NO_PAD.encode(&ciphertext));
  Ok(())
 }