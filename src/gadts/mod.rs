//! # Generalized Algebraic Data Types (GADTs)
//!
//! This module demonstrates GADT-like patterns in Rust using associated types
//! with lifetime parameters. While Rust doesn't have true GADTs, we can achieve
//! similar patterns using associated types and generic associated types (GATs).
//!
//! ## Features
//!
//! - Generic associated types with lifetime parameters
//! - Type-safe collection factories
//! - Container abstractions with associated types
//!
//! ## Examples
//!
//! ```rust
//! use aabel_types::gadts::*;
//!
//! // Create a collection factory
//! let factory = VecFactory(vec![1, 2, 3]);
//! let collection = factory.create_collection();
//! let mut iter = factory.create_iterator();
//!
//! assert_eq!(iter.next(), Some(&1));
//! ```

mod closures;
mod collection_factory;
mod container;

pub use collection_factory::*;
pub use container::*;
pub use closures::*;
