use des::Des;
use des::cipher::{NewBlockCipher, BlockEncrypt, BlockDecrypt};

/// Combine keys into a single 8-byte DES key
fn combine_keys(keys: &[&str]) -> Vec<u8> {
    let mut combined = Vec::new();
    for key in keys {
        combined.extend_from_slice(key.as_bytes());
    }

    // Ensure length is a multiple of 8
    if combined.len() < 8 {
        combined.resize(8, 0);
    } else if combined.len() % 8 != 0 {
        combined.truncate((combined.len() / 8) * 8);
    }

    // Return only the first 8 bytes for DES
    combined.into_iter().take(8).collect()
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

/// Encrypt data using DES (support single, double, and triple DES)
pub fn str_enc(data: &str, first_key: &str, second_key: &str, third_key: &str) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let plain_bytes = data.as_bytes();
    let padded_bytes = pkcs7_pad(plain_bytes, 8);

    let encrypted = if !first_key.is_empty() && !second_key.is_empty() && !third_key.is_empty() {
        // Triple DES (3DES) - E-D-E mode
        let key1 = combine_keys(&[first_key]);
        let key2 = combine_keys(&[second_key]);
        let key3 = combine_keys(&[third_key]);

        // Convert Vec<u8> to [u8; 8] arrays
        let mut key1_array = [0u8; 8];
        let mut key2_array = [0u8; 8];
        let mut key3_array = [0u8; 8];

        key1_array.copy_from_slice(&key1);
        key2_array.copy_from_slice(&key2);
        key3_array.copy_from_slice(&key3);

        // E-D-E: Encrypt with key1, Decrypt with key2, Encrypt with key3
        let cipher1 = Des::new(&key1_array.into());
        let mut block = padded_bytes.to_vec();

        for chunk in block.chunks_mut(8) {
            cipher1.encrypt_block(chunk.into());
        }

        let cipher2 = Des::new(&key2_array.into());
        for chunk in block.chunks_mut(8) {
            cipher2.decrypt_block(chunk.into());
        }

        let cipher3 = Des::new(&key3_array.into());
        for chunk in block.chunks_mut(8) {
            cipher3.encrypt_block(chunk.into());
        }

        block
    } else if !first_key.is_empty() && !second_key.is_empty() {
        // Double DES
        let key1 = combine_keys(&[first_key]);
        let key2 = combine_keys(&[second_key]);

        let mut key1_array = [0u8; 8];
        let mut key2_array = [0u8; 8];

        key1_array.copy_from_slice(&key1);
        key2_array.copy_from_slice(&key2);

        let cipher1 = Des::new(&key1_array.into());
        let mut block = padded_bytes.to_vec();

        for chunk in block.chunks_mut(8) {
            cipher1.encrypt_block(chunk.into());
        }

        let cipher2 = Des::new(&key2_array.into());
        for chunk in block.chunks_mut(8) {
            cipher2.encrypt_block(chunk.into());
        }

        block
    } else if !first_key.is_empty() {
        // Single DES
        let key = combine_keys(&[first_key]);

        let mut key_array = [0u8; 8];
        key_array.copy_from_slice(&key);

        let cipher = Des::new(&key_array.into());
        let mut block = padded_bytes.to_vec();

        for chunk in block.chunks_mut(8) {
            cipher.encrypt_block(chunk.into());
        }

        block
    } else {
        return Ok(String::new());
    };

    Ok(hex::encode(encrypted).to_uppercase())
}

/// Decrypt data using DES (support single, double, and triple DES)
#[allow(dead_code)]
pub fn str_dec(data: &str, first_key: &str, second_key: &str, third_key: &str) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let cipher_bytes = hex::decode(data).map_err(|e| format!("Hex decode error: {}", e))?;

    let decrypted = if !first_key.is_empty() && !second_key.is_empty() && !third_key.is_empty() {
        // Triple DES (3DES) - Decryption is D-E-D (reverse of E-D-E)
        let key1 = combine_keys(&[first_key]);
        let key2 = combine_keys(&[second_key]);
        let key3 = combine_keys(&[third_key]);

        let mut key1_array = [0u8; 8];
        let mut key2_array = [0u8; 8];
        let mut key3_array = [0u8; 8];

        key1_array.copy_from_slice(&key1);
        key2_array.copy_from_slice(&key2);
        key3_array.copy_from_slice(&key3);

        // D-E-D: Decrypt with key3, Encrypt with key2, Decrypt with key1
        let cipher3 = Des::new(&key3_array.into());
        let mut block = cipher_bytes.to_vec();

        for chunk in block.chunks_mut(8) {
            cipher3.decrypt_block(chunk.into());
        }

        let cipher2 = Des::new(&key2_array.into());
        for chunk in block.chunks_mut(8) {
            cipher2.encrypt_block(chunk.into());
        }

        let cipher1 = Des::new(&key1_array.into());
        for chunk in block.chunks_mut(8) {
            cipher1.decrypt_block(chunk.into());
        }

        block
    } else if !first_key.is_empty() && !second_key.is_empty() {
        // Double DES
        let key1 = combine_keys(&[first_key]);
        let key2 = combine_keys(&[second_key]);

        let mut key1_array = [0u8; 8];
        let mut key2_array = [0u8; 8];

        key1_array.copy_from_slice(&key1);
        key2_array.copy_from_slice(&key2);

        let cipher2 = Des::new(&key2_array.into());
        let mut block = cipher_bytes.to_vec();

        for chunk in block.chunks_mut(8) {
            cipher2.decrypt_block(chunk.into());
        }

        let cipher1 = Des::new(&key1_array.into());
        for chunk in block.chunks_mut(8) {
            cipher1.decrypt_block(chunk.into());
        }

        block
    } else if !first_key.is_empty() {
        // Single DES
        let key = combine_keys(&[first_key]);

        let mut key_array = [0u8; 8];
        key_array.copy_from_slice(&key);

        let cipher = Des::new(&key_array.into());
        let mut block = cipher_bytes.to_vec();

        for chunk in block.chunks_mut(8) {
            cipher.decrypt_block(chunk.into());
        }

        block
    } else {
        return Ok(String::new());
    };

    // Try to remove padding
    let unpadded = pkcs7_unpad(&decrypted).unwrap_or(decrypted);

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
