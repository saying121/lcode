use std::fs::read_to_string;

use base64::{engine::general_purpose, Engine};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct LocalState {
    pub os_crypt: OsCrypt,
}
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
struct OsCrypt {
    pub audit_enabled: bool,
    pub encrypted_key: String,
}

pub async fn get_pass() -> Result<Vec<u8>> {
    let mut path = dirs::data_local_dir().expect("get path failed");
    path.push(r#"Google\Chrome\User Data\Local State"#);
    let string_str = read_to_string(path).into_diagnostic()?;
    let local_state: LocalState = serde_json::from_str(&string_str).into_diagnostic()?;
    let encrypted_key = general_purpose::STANDARD
        .decode(local_state.os_crypt.encrypted_key)
        .into_diagnostic()?;
    let key = encrypted_key[5..].to_vec();

    let key = decrypt_data_key(key).await?;

    Ok(key)
}

use std::ptr::null_mut;

use winapi::um::{
    dpapi::CryptUnprotectData, winbase::LocalFree, wincrypt::CRYPTOAPI_BLOB, winnt::HANDLE,
};

pub async fn decrypt_data_key(mut unencoded_key: Vec<u8>) -> Result<Vec<u8>> {
    let mut data_in = CRYPTOAPI_BLOB {
        cbData: unencoded_key.len() as u32,
        pbData: unencoded_key.as_mut_ptr(),
    };
    let mut data_out = CRYPTOAPI_BLOB { cbData: 0, pbData: null_mut() };
    unsafe {
        CryptUnprotectData(
            &mut data_in,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut data_out,
        );

        let bytes = Vec::from_raw_parts(
            data_out.pbData,
            data_out.cbData as usize,
            data_out.cbData as usize,
        );
        LocalFree(data_out.pbData as HANDLE);
        Ok(bytes)
    }
}
