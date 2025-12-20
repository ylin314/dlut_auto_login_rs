use des::cipher::{BlockDecrypt, BlockEncrypt, NewBlockCipher};
use des::TdesEde3;

/// Get an 8-byte DES key from a string
fn get_des_key(key: &str) -> [u8; 8] {
    let mut res = [0u8; 8];
    let bytes = key.as_bytes();
    let len = bytes.len().min(8);
    res[..len].copy_from_slice(&bytes[..len]);
    res
}

/// PKCS#7 padding
fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded = data.to_vec();
    let padding_length = block_size - (data.len() % block_size);
    padded.extend(std::iter::repeat(padding_length as u8).take(padding_length));
    padded
}

/// Remove PKCS#7 padding
#[allow(dead_code)]
fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("Data is empty".to_string());
    }

    let padding_length = data[data.len() - 1] as usize;
    if padding_length > 8 || padding_length == 0 {
        return Err("Invalid padding".to_string());
    }

    Ok(data[..data.len() - padding_length].to_vec())
}

/// Encrypt data using Triple DES (3DES) - E-D-E mode
pub fn str_enc(
    data: &str,
    first_key: &str,
    second_key: &str,
    third_key: &str,
) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let mut key = [0u8; 24];
    key[0..8].copy_from_slice(&get_des_key(first_key));
    key[8..16].copy_from_slice(&get_des_key(second_key));
    key[16..24].copy_from_slice(&get_des_key(third_key));

    let cipher = TdesEde3::new(&key.into());
    let mut block = pkcs7_pad(data.as_bytes(), 8);

    for chunk in block.chunks_mut(8) {
        cipher.encrypt_block(chunk.into());
    }

    Ok(hex::encode(block).to_uppercase())
}

/// Decrypt data using Triple DES (3DES) - D-E-D mode
#[allow(dead_code)]
pub fn str_dec(
    data: &str,
    first_key: &str,
    second_key: &str,
    third_key: &str,
) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let mut key = [0u8; 24];
    key[0..8].copy_from_slice(&get_des_key(first_key));
    key[8..16].copy_from_slice(&get_des_key(second_key));
    key[16..24].copy_from_slice(&get_des_key(third_key));

    let cipher = TdesEde3::new(&key.into());
    let mut block = hex::decode(data).map_err(|e| format!("Hex decode error: {}", e))?;

    for chunk in block.chunks_mut(8) {
        cipher.decrypt_block(chunk.into());
    }

    let unpadded = pkcs7_unpad(&block).unwrap_or(block);
    Ok(String::from_utf8_lossy(&unpadded).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_enc_dec() {
        let data = "test";
        let encrypted = str_enc(data, "1", "2", "3").unwrap();
        let decrypted = str_dec(&encrypted, "1", "2", "3").unwrap();
        assert_eq!(data, decrypted);
    }
}
