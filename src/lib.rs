#![deny(missing_docs)]

//! Subversive is a cryptographic toolkit and network protocol implementation.
//!
//! # Features
//! - Cryptographic primitives (hashing, signatures)
//! - Serialization formats (Base36, Base58, Base64)
//! - Logging utilities with colored output
//! - Proof of concept implementations

//pub mod address;
//pub mod crypto;

/// Cryptographic hashing implementations and types
pub mod hashing;

/// Logging utilities with colored output and formatting
pub mod logging;

/// Serialization formats and utilities
pub mod serialise;

#[cfg(test)]
/// Proof of concept implementations and examples
pub mod poc;
