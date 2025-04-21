//! # aabel-types
//!
//! A Rust library exploring advanced type system concepts and patterns.
//!
//! This library provides implementations and examples of various advanced type system
//! features in Rust, including:
//!
//! - Advanced lifetime patterns
//! - Generalized Algebraic Data Types (GADTs)
//! - Phantom types for type-safe APIs
//! - Typeclass implementations
//! - Zero-sized types (ZSTs)
//!
//! ## Modules
//!
//! - [`advanced_lifetime`] - Advanced lifetime patterns and techniques
//! - [`gadts`] - Generalized Algebraic Data Types implementation
//! - [`phantom_types`] - Phantom type patterns and usage
//! - [`typeclass`] - Typeclass implementations and patterns
//! - [`zsts`] - Zero-sized type patterns and optimizations
//!
//! ## Examples
//!
//! ```rust
//! use aabel_types::phantom_types::*;
//!
//! // Create a type-safe API using phantom types
//! let token = Token::<Unvalidated>::new("my_token".to_string());
//! let validated = token.validate().unwrap();
//! println!("Valid token: {}", validated.use_validated());
//! ```
//!
//! ## Safety
//!
//! This library uses Rust's type system to provide compile-time guarantees
//! without runtime overhead. All unsafe code is clearly marked and documented.

mod advanced_lifetime;
mod gadts;
mod phantom_types;
mod typeclass;
mod zsts;

pub use advanced_lifetime::*;
pub use gadts::*;
pub use phantom_types::*;
pub use typeclass::*;
pub use zsts::*;
