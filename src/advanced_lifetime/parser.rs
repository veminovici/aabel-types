/// A trait for parsing input with advanced lifetime handling.
///
/// This trait demonstrates the use of higher-ranked trait bounds (HRTBs)
/// to safely handle borrowed data in a generic way.
///
/// # Examples
///
/// ```rust
/// use aabel_types::advanced_lifetime::*;
///
/// let parser = SimpleParser;
/// let length = parser.parse(|s| s.len());
/// assert_eq!(length, 13);
/// ```
pub trait Parser {
    /// Parses input using a provided closure.
    ///
    /// The closure can capture the input string with any lifetime, thanks to
    /// the higher-ranked trait bound `for<'a>`.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a string slice and returns some output
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aabel_types::advanced_lifetime::*;
    ///
    /// let parser = SimpleParser;
    /// let result = parser.parse(|s| s.to_uppercase());
    /// assert_eq!(result, "HELLO, WORLD!");
    /// ```
    fn parse<F, O>(&self, f: F) -> O
    where
        F: for<'a> FnOnce(&'a str) -> O;
}

/// A simple parser implementation that demonstrates advanced lifetime patterns.
///
/// This struct provides a concrete implementation of the `Parser` trait,
/// showing how to safely handle borrowed data with complex lifetime requirements.
#[derive(Debug, Default)]
pub struct SimpleParser;

impl Parser for SimpleParser {
    /// Implementation of the parse method for SimpleParser.
    ///
    /// This implementation demonstrates how to use higher-ranked trait bounds
    /// to safely handle borrowed data in a generic context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aabel_types::advanced_lifetime::*;
    ///
    /// let parser = SimpleParser;
    /// let result = parser.parse(|s| s.contains("world"));
    /// assert!(result);
    /// ```
    fn parse<F, O>(&self, f: F) -> O
    where
        F: for<'a> FnOnce(&'a str) -> O,
    {
        let input = "Hello, world!";
        f(input)
    }
}
