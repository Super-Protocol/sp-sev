// SPDX-License-Identifier: Apache-2.0

//! Hashing helpers for the measurement path, abstracted over the crypto backend.
//!
//! The `openssl` feature uses OpenSSL (native builds, e.g. the napi binding),
//! while `crypto_nossl` uses the pure-Rust `sha2` crate so the measurement can
//! be compiled for `wasm32` / the browser. Both produce identical digests.

/// SHA-384 digest of `data` (48 bytes).
#[cfg(feature = "openssl")]
pub(crate) fn sha384(data: &[u8]) -> [u8; 48] {
    openssl::sha::sha384(data)
}

/// SHA-256 digest of `data` (32 bytes).
#[cfg(feature = "openssl")]
pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
    openssl::sha::sha256(data)
}

/// SHA-384 digest of `data` (48 bytes).
#[cfg(all(feature = "crypto_nossl", not(feature = "openssl")))]
pub(crate) fn sha384(data: &[u8]) -> [u8; 48] {
    use sha2::{Digest, Sha384};
    Sha384::digest(data).into()
}

/// SHA-256 digest of `data` (32 bytes).
#[cfg(all(feature = "crypto_nossl", not(feature = "openssl")))]
pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(data).into()
}
