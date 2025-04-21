/// A trait for containers that can provide items with proper lifetime handling.
///
/// This trait demonstrates how to use associated types with lifetime parameters
/// to create type-safe container abstractions.
///
/// # Examples
///
/// ```rust
/// use aabel_types::gadts::*;
///
/// let vec = vec![1, 2, 3];
/// let item = vec.get_item();
/// assert_eq!(item, Some(&1));
/// ```
trait Container {
    /// The type of items stored in this container.
    ///
    /// This associated type is generic over a lifetime parameter, allowing
    /// the container to properly handle borrowed data.
    type Item<'a>
    where
        Self: 'a;

    /// Retrieves an item from the container.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the container
    ///
    /// # Returns
    ///
    /// An optional reference to an item in the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aabel_types::gadts::*;
    ///
    /// let vec = vec![1, 2, 3];
    /// let item = vec.get_item();
    /// assert_eq!(item, Some(&1));
    /// ```
    fn get_item<'a>(&'a self) -> Option<Self::Item<'a>>;
}

impl<T> Container for Vec<T> {
    type Item<'a>
        = &'a T
    where
        Self: 'a;

    fn get_item<'a>(&'a self) -> Option<Self::Item<'a>> {
        self.first()
    }
}

/// Demonstrates the usage of the Container trait with Vec.
///
/// This function shows how to use the Container trait with a concrete type
/// and verifies its behavior.
///
/// # Examples
///
/// ```rust
/// use aabel_types::gadts::*;
///
/// main_gadts(); // Prints "Some(1)"
/// ```
pub fn main_gadts() {
    let vec = vec![1, 2, 3];
    let item = vec.get_item();
    assert_eq!(item, Some(&1));
    println!("{:?}", item);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the get_item functionality of the Container trait.
    ///
    /// This test verifies that the Container trait correctly retrieves
    /// the first item from a Vec.
    #[test]
    fn test_get_item() {
        let xs = vec![1, 2, 3];
        let item = xs.get_item();
        assert_eq!(item, Some(&1));
    }
}
