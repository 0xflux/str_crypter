# str_crypter

`str_crypter` is a Rust library for XOR encryption and decryption of strings. It provides a macro to XOR encrypt strings from plaintext at compile time using a single byte key. When building in release mode, any plaintext strings encrypted with the macro `sc!()` will not be present in the binary. At runtime, the macro expands to decrypt the encrypted string.

### Version info

This was originally turned into a 2.0 crate, however after additional testing the crate was not working as intended, so I have yanked those versions and reverted it to an older working copy under v1. The changes made in 2.0 weren't particularly groundbreaking nor did it add extra functionality, so I have no issues yanking it and working on it again when I have some more time.

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