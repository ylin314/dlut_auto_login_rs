use des::cipher::generic_array::GenericArray;
use des::cipher::{BlockDecrypt, BlockEncrypt, NewBlockCipher};
use des::Des;

/// 将密钥组合并确保长度为8字节（DES密钥长度）
fn combine_keys(key: &str) -> GenericArray<u8, des::cipher::consts::U8> {
    let key_bytes = key.as_bytes();
    let mut result = [0u8; 8];

    // 复制密钥字节，最多8个
    let copy_len = key_bytes.len().min(8);
    result[..copy_len].copy_from_slice(&key_bytes[..copy_len]);

    // 如果密钥长度不足8，剩余部分已经是0（用\0填充）
    GenericArray::clone_from_slice(&result)
}

/// PKCS7填充
fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(vec![padding_len as u8; padding_len]);
    padded
}

/// 使用DES加密（支持Triple DES）
/// 参数: data (明文字符串), first_key, second_key, third_key
/// 返回值: 十六进制加密字符串（大写）
pub fn str_enc(data: &str, first_key: &str, second_key: &str, third_key: &str) -> String {
    if data.is_empty() {
        return String::new();
    }

    // 将字符串转换为字节并进行PKCS7填充
    let plain_bytes = pkcs7_pad(data.as_bytes(), 8);

    let encrypted = if !first_key.is_empty() && !second_key.is_empty() && !third_key.is_empty() {
        // Triple DES (3DES) - E-D-E 模式
        let key1 = combine_keys(first_key);
        let key2 = combine_keys(second_key);
        let key3 = combine_keys(third_key);

        let cipher1 = Des::new(&key1);
        let cipher2 = Des::new(&key2);
        let cipher3 = Des::new(&key3);

        // 对每个8字节块进行加密
        let mut result = plain_bytes.clone();
        for chunk in result.chunks_mut(8) {
            let block = GenericArray::from_mut_slice(chunk);
            // E-D-E: 加密 -> 解密 -> 加密
            cipher1.encrypt_block(block);
            cipher2.decrypt_block(block);
            cipher3.encrypt_block(block);
        }
        result
    } else if !first_key.is_empty() && !second_key.is_empty() {
        // Double DES
        let key1 = combine_keys(first_key);
        let key2 = combine_keys(second_key);

        let cipher1 = Des::new(&key1);
        let cipher2 = Des::new(&key2);

        let mut result = plain_bytes.clone();
        for chunk in result.chunks_mut(8) {
            let block = GenericArray::from_mut_slice(chunk);
            cipher1.encrypt_block(block);
            cipher2.encrypt_block(block);
        }
        result
    } else if !first_key.is_empty() {
        // Single DES
        let key = combine_keys(first_key);
        let cipher = Des::new(&key);

        let mut result = plain_bytes.clone();
        for chunk in result.chunks_mut(8) {
            let block = GenericArray::from_mut_slice(chunk);
            cipher.encrypt_block(block);
        }
        result
    } else {
        return String::new();
    };

    // 转换为大写十六进制字符串
    hex::encode_upper(encrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_enc_single_key() {
        // 测试单密钥加密
        let result = str_enc("test", "key12345", "", "");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_str_enc_triple_key() {
        // 测试三重DES加密
        let result = str_enc("test", "key1", "key2", "key3");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_str_enc_empty_data() {
        // 测试空数据
        let result = str_enc("", "key1", "key2", "key3");
        assert!(result.is_empty());
    }

    #[test]
    fn test_str_enc_compatibility_with_python() {
        // 测试与Python实现的兼容性
        // Python: str_enc('testdata123', '1', '2', '3') == '2E155BA67E5D4D70350A99EA5F209F8D'
        let result = str_enc("testdata123", "1", "2", "3");
        assert_eq!(result, "2E155BA67E5D4D70350A99EA5F209F8D");
    }

    #[test]
    fn test_str_enc_no_key() {
        // 测试无密钥
        let result = str_enc("test", "", "", "");
        assert!(result.is_empty());
    }
}
