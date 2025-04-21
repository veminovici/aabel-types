//! # Phantom Types Module
//!
//! This module demonstrates the use of phantom types in Rust to create type-safe APIs
//! and enforce state transitions at compile time.
//!
//! Phantom types are zero-sized types used to carry additional type information
//! that is checked at compile time but doesn't exist at runtime. They're particularly
//! useful for:
//!
//! - Creating type-safe APIs
//! - Enforcing state machine transitions
//! - Adding compile-time guarantees without runtime overhead
//!
//! ## Example
//!
//! ```rust
//! use aabel_types::phantom_types::*;
//!
//! // Create an unvalidated token
//! let token = Token::<Unvalidated>::new("my_token".to_string());
//!
//! // Validate the token (can fail)
//! match token.validate() {
//!     Ok(valid_token) => {
//!         // Now we can safely use the validated token
//!         println!("Token is valid: {}", valid_token.use_validated());
//!     }
//!     Err(e) => println!("Validation failed: {:?}", e),
//! }
//! ```

mod state_machine;
pub use state_machine::*;
