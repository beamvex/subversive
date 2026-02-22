#![deny(missing_docs)]

//! Subversive is a cryptographic toolkit and network protocol implementation.
//!
//! # Features
//! - Cryptographic primitives (hashing, signatures)
//! - Serialization formats (Base36, Base58, Base64)
//! - Logging utilities with colored output
//! - Proof of concept implementations

//pub mod address;

/// Cryptographic primitives and related types.
pub mod crypto;

#[cfg(test)]
/// Proof of concept implementations and examples
pub mod poc;
