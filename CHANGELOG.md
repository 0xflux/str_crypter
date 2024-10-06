# Version 2.0

## Added

Optional Key Parameter: The sc! macro now accepts an optional key. If no key is provided, it automatically uses a hashed version of the line number as the key, enhancing ease of use.

Dynamic String Length Handling: The macro now dynamically adjusts to the length of the input string, removing the previous fixed-size array limitation.

## Changed

Inlined Decryption Logic: Moved the decryption logic inside the sc! macro, eliminating the need for an external decrypt_string function.

Simplified Return Type: The macro now returns a String directly instead of a Result, as errors are impossible with string literals and reversible XOR operations.

Non-Zero Key Enforcement: Adjusted the key to ensure it is never zero, defaulting to 1 if a zero key is provided.

Removed UTF-8 Validation: Skipped UTF-8 validation during decryption using String::from_utf8_unchecked, since the process guarantees valid UTF-8 output.

## Removed

Fixed-Length Array Limitation: Eliminated the fixed-size array [u8; 256], allowing the macro to handle strings of any length.

## Fixed

Key Collision Handling: Implemented a hash function for the line number to handle cases where the line number exceeds 255, reducing the likelihood of key collisions when using the line number as a key.

Zero Key Handling: Ensured that a provided key of zero is adjusted to a non-zero value to maintain encryption integrity.

## Testing

Expanded Test Coverage: Updated tests to cover the new functionality, including:

1) Using the macro without specifying a key.
2) Handling of empty strings.
3) Special characters and Unicode support.
4) Zero key adjustment verification.