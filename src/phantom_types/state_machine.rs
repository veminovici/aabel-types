use std::marker::PhantomData;

/// A token that can be in different states, enforced by the type system.
///
/// The `State` type parameter is a phantom type that represents the current state
/// of the token. This allows the compiler to enforce that certain operations can
/// only be performed on tokens in specific states.
///
/// # Type Parameters
///
/// * `State` - The current state of the token (Unvalidated or Validated)
///
/// # Examples
///
/// ```
/// use aabel_types::phantom_types::*;
///
/// let token = Token::<Unvalidated>::new("my_token".to_string());
/// let validated = token.validate().unwrap();
/// println!("Valid token: {}", validated.use_validated());
/// ```
pub struct Token<State> {
    value: String,
    _state: PhantomData<State>,
}

/// Represents an unvalidated token state.
///
/// This is a marker type used as a phantom type parameter to `Token`.
pub struct Unvalidated;

/// Represents a validated token state.
///
/// This is a marker type used as a phantom type parameter to `Token`.
pub struct Validated;

/// Errors that can occur during token validation.
#[derive(Debug)]
pub enum ValidationError {
    /// The token is too short (less than 4 characters)
    TooShort,
    /// The token has an invalid format
    InvalidFormat,
}

impl Token<Unvalidated> {
    /// Creates a new unvalidated token.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value of the token
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_types::phantom_types::*;
    ///
    /// let token = Token::<Unvalidated>::new("my_token".to_string());
    /// ```
    pub fn new(value: String) -> Self {
        Token {
            value,
            _state: PhantomData,
        }
    }

    /// Validates the token and returns a validated token if successful.
    ///
    /// This method performs validation checks on the token and returns either
    /// a validated token or a validation error.
    ///
    /// # Returns
    ///
    /// * `Ok(Token<Validated>)` - If validation succeeds
    /// * `Err(ValidationError)` - If validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_types::phantom_types::*;
    ///
    /// let token = Token::<Unvalidated>::new("my_token".to_string());
    /// match token.validate() {
    ///     Ok(valid) => println!("Token is valid"),
    ///     Err(e) => println!("Validation failed: {:?}", e),
    /// }
    /// ```
    pub fn validate(self) -> Result<Token<Validated>, ValidationError> {
        // Perform validation
        if self.value.len() > 3 {
            Ok(Token {
                value: self.value,
                _state: PhantomData,
            })
        } else {
            Err(ValidationError::TooShort)
        }
    }
}

impl Token<Validated> {
    /// Uses the validated token.
    ///
    /// This method can only be called on validated tokens, providing a type-safe
    /// way to access the token's value.
    ///
    /// # Returns
    ///
    /// A reference to the token's string value.
    ///
    /// # Examples
    ///
    /// ```
    /// use aabel_types::phantom_types::*;
    ///
    /// let token = Token::<Unvalidated>::new("my_token".to_string());
    /// let validated = token.validate().unwrap();
    /// println!("Token value: {}", validated.use_validated());
    /// ```
    pub fn use_validated(&self) -> &str {
        &self.value
    }
}
