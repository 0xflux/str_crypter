# str_crypter

`str_crypter` is a Rust library for XOR encryption and decryption of strings. It provides a macro to XOR encrypt strings from plaintext at compile time using a single byte key. When building in release mode, any plaintext strings encrypted with the macro `sc!()` will not be present in the binary. At runtime, the macro expands to decrypt the encrypted string.

## Usage

```rust
use str_crypter::{decrypt_string, sc};

fn main() {
    let encrypted_str: String = match sc!("Hello world!", 20) {
        Ok(s) => s,
        Err(e) => panic!("Decryption failed: {:?}", e),
    };

    println!("Decrypted string: {}", encrypted_str);
}
```

## Installation

Add `str_crypter` to your `Cargo.toml`:

```toml
[dependencies]
str_crypter = "1.0.3"
```

Or use the command `cargo add str_crypter`