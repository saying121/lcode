use aes_gcm::aead::AeadMutInPlace;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use miette::{IntoDiagnostic, Result};

pub fn decrypt_cookies(encrypted_value: &mut Vec<u8>, pass: &[u8]) -> Result<String> {
    let iv = &encrypted_value[3..15];
    let encrypted_value = &encrypted_value[15..];

    let cipher = Aes256Gcm::new(pass.into());

        let payload = encrypted_value.into();
    cipher.decrypt_in_place(iv.into(), payload.msg, &encrypted_value[15..])

    let Ok(decrypted) = cipher.decrypt(iv.into(), encrypted_value)
    else {
        miette::bail!("decrypt failed");
    };

    String::from_utf8(decrypted).into_diagnostic()
}
