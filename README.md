# str_crypter

`str_crypter` is a Rust library for XOR encryption and decryption of strings. It provides a macro to XOR encrypt strings from plaintext at compile time using a single byte key. When building in release mode, any plaintext strings encrypted with the macro `sc!()` will not be present in the binary. At runtime, the macro expands to decrypt the encrypted string.

## Usage

```rust
use str_crypter::sc;

fn main() {
    let decrypted_str = sc!("Hello world!", 20);
    println!("Decrypted string: {}", decrypted_str);
}
```

## Installation

Add `str_crypter` to your `Cargo.toml`:

```toml
[dependencies]
str_crypter = "2.0.1"
```

Or use the command `cargo add str_crypter`