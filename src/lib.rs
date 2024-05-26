use std::string::FromUtf8Error;

/// XOR decrypts a given encrypted byte slice with the provided key.
///
/// # Arguments
///
/// * `encrypted` - A slice of bytes that holds the encrypted data.
/// * `key` - A byte (u8) used as the key for the XOR decryption.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(String)` containing the decrypted string if the conversion succeeds.
/// - `Err(FromUtf8Error)` if the bytes are not valid UTF-8.
pub fn decrypt_string(encrypted: &[u8], key: u8) -> Result<String, FromUtf8Error> {
    let decrypted_bytes: Vec<u8> = encrypted.iter().map(|&b| b ^ key).collect();
    String::from_utf8(decrypted_bytes)
}

/// Macro to encrypt a string at compile time and decrypt it at runtime.
///
/// # Arguments
///
/// * `$s` - The string literal to be encrypted.
/// * `$key` - The byte (u8) key used for XOR encryption.
///
/// # Returns
/// 
/// * `Result<String, FromUtf8Error>`
/// 
/// # Example
///
/// ```rust
/// use str_crypter::{decrypt_string, sc};
///
/// fn main() {
///     let encrypted_str: String = match sc!("Hello world!", 20) {
///         Ok(s) => s,
///         Err(e) => panic!("Decryption failed: {:?}", e),
///     };
/// 
///     println!("Decrypted string: {}", encrypted_str);
/// }
/// ```
#[macro_export]
macro_rules! sc {
    ($s:expr, $key:expr) => {{
        // Function to perform compile-time XOR encryption
        const fn xor_encrypt_const(input: &str, key: u8) -> [u8; 256] {
            // Recursive function to apply XOR encryption
            const fn xor_encrypt_recursive(input: &[u8], key: u8, i: usize, mut acc: [u8; 256]) -> [u8; 256] {
                if i == input.len() {
                    acc
                } else {
                    acc[i] = input[i] ^ key;
                    xor_encrypt_recursive(input, key, i + 1, acc)
                }
            }
            
            // Call the function
            xor_encrypt_recursive(input.as_bytes(), key, 0, [0; 256])
        }

        // Encrypt the input at compile time
        const ENCRYPTED: [u8; 256] = xor_encrypt_const($s, $key);

        // Decrypt the encrypted string at runtime
        decrypt_string(&ENCRYPTED[..$s.len()], $key)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sc_macro_success() {
        const KEY: u8 = 20;
        let decrypted = sc!("Hello, World!", KEY);
        assert_eq!(decrypted.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_sc_macro_empty_string() {
        const KEY: u8 = 20;
        let decrypted = sc!("", KEY);
        assert_eq!(decrypted.unwrap(), "");
    }

    #[test]
    fn test_sc_macro_special_characters() {
        const KEY: u8 = 20;
        let decrypted = sc!("!@#$%^&*()", KEY);
        assert_eq!(decrypted.unwrap(), "!@#$%^&*()");
    }
}