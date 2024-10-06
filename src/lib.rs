/// Macro to encrypt a string at compile time and decrypt it at runtime.
/// If no key is provided, it uses the line number as the key.
///
/// # Arguments
///
/// * `$s` - The string literal to be encrypted.
/// * `$key` - *(Optional)* The byte (u8) key used for XOR encryption.
///
/// # Returns
/// 
/// * `String`
/// 
/// # Example
///
/// ```rust
/// use str_crypter::sc;
///
/// fn main() {
///     let decrypted_str = sc!("Hello world!", 20);
///     println!("Decrypted string: {}", decrypted_str);
/// }
/// ```
#[macro_export]
macro_rules! sc {
    // if the string is provided with no ln number, use a hashed ln number as the key
    ($s:expr) => {{
        // hash function to generate a non-zero u8 key from the line number
        const fn hash_line_number(line: u32) -> u8 {
            let mut hash = line.wrapping_mul(31) as u8;
            if hash == 0 {
                hash = 1; // key is not zero
            }
            hash
        }
        $crate::sc!($s, hash_line_number(line!()))
    }};

    // macro definition with string and key
    ($s:expr, $key:expr) => {{
        // ensure the key is not zero
        const K: u8 = if $key == 0 { 1 } else { $key };
        const N: usize = $s.len();

        // encrypt the input at compile time
        const ENCRYPTED: [u8; N] = {
            let input = $s.as_bytes();
            let mut encrypted = [0u8; N];
            let mut i = 0;
            while i < N {
                encrypted[i] = input[i] ^ K.wrapping_add(i as u8);
                i += 1;
            }
            encrypted
        };

        // decrypt the encrypted string at runtime, UTF checking not required
        unsafe {
            let mut decrypted = [0u8; N];
            let mut i = 0;
            while i < N {
                decrypted[i] = ENCRYPTED[i] ^ K.wrapping_add(i as u8);
                i += 1;
            }
            String::from_utf8_unchecked(decrypted.to_vec())
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sc_macro_success() {
        const KEY: u8 = 20;
        let decrypted = sc!("Hello, World!", KEY);
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_sc_macro_empty_string() {
        const KEY: u8 = 20;
        let decrypted = sc!("", KEY);
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_sc_macro_special_characters() {
        const KEY: u8 = 20;
        let decrypted = sc!("!@#$%^&*()", KEY);
        assert_eq!(decrypted, "!@#$%^&*()");
    }

    #[test]
    fn test_sc_macro_zero_key() {
        let decrypted = sc!("Test with zero key", 0);
        assert_eq!(decrypted, "Test with zero key");
    }

    #[test]
    fn test_sc_macro_with_line_key() {
        let decrypted = sc!("Testing no key entered!");
        let expected = "Testing no key entered!";
        assert_eq!(decrypted, expected);
    }

    #[test]
    fn test_sc_macro_unicode_characters() {
        let decrypted = sc!("こんにちは世界", 42);
        assert_eq!(decrypted, "こんにちは世界");
    }
}