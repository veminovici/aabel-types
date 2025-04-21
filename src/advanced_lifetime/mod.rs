//! # Advanced Lifetime Patterns
//!
//! This module demonstrates advanced lifetime patterns in Rust, particularly focusing on
//! higher-ranked trait bounds (HRTBs) and lifetime elision rules.
//!
//! ## Features
//!
//! - Higher-ranked trait bounds with `for<'a>` syntax
//! - Lifetime elision in trait implementations
//! - Safe abstraction over borrowed data
//!
//! ## Examples
//!
//! ```rust
//! use aabel_types::advanced_lifetime::*;
//!
//! let parser = SimpleParser;
//! let result = parser.parse(|s| s.len());
//! println!("Parsed length: {}", result);
//! ```

mod parser;
pub use parser::*;
