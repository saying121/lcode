use aes::cipher::{block_padding, BlockDecryptMut, KeyIvInit};
use miette::Result;
use pbkdf2::pbkdf2_hmac;

pub fn decrypt_cookies(be_decrypte: &mut Vec<u8>, pass: &[u8]) -> Result<()> {
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

    let mut key = [0_u8; 16];
    let iv = [32_u8; 16];

    pbkdf2_hmac::<sha1::Sha1>(pass, b"saltysalt", 1, &mut key);

    let decrypter = Aes128CbcDec::new(&key.into(), &iv.into());

    if decrypter
        .decrypt_padded_mut::<block_padding::NoPadding>(&mut be_decrypte[3..])
        .is_err()
    {
        miette::bail!("decrypt failed")
    }

    be_decrypte.retain(|v| v >= &32);

    Ok(())
}
