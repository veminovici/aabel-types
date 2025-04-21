/// A trait for creating collections and their iterators with proper lifetime handling.
///
/// This trait demonstrates how to use generic associated types (GATs) to create
/// type-safe collection factories that handle lifetimes correctly.
///
/// # Examples
///
/// ```rust
/// use aabel_types::gadts::*;
///
/// let factory = VecFactory(vec![1, 2, 3]);
/// let collection = factory.create_collection();
/// let mut iter = factory.create_iterator();
///
/// assert_eq!(iter.next(), Some(&1));
/// ```
pub trait CollectionFactory {
    /// The type of collection produced by this factory.
    ///
    /// This associated type is generic over a lifetime parameter, allowing
    /// the collection to properly handle borrowed data.
    type Collection<'a>
    where
        Self: 'a;

    /// The type of iterator produced by this factory.
    ///
    /// This associated type is generic over a lifetime parameter, ensuring
    /// that iterators can safely borrow from their collections.
    type Iterator<'a>
    where
        Self: 'a;

    /// Creates a new collection instance.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the factory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aabel_types::gadts::*;
    ///
    /// let factory = VecFactory(vec![1, 2, 3]);
    /// let collection = factory.create_collection();
    /// assert_eq!(collection.len(), 3);
    /// ```
    fn create_collection<'a>(&'a self) -> Self::Collection<'a>;

    /// Creates a new iterator over the collection.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the factory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aabel_types::gadts::*;
    ///
    /// let factory = VecFactory(vec![1, 2, 3]);
    /// let mut iter = factory.create_iterator();
    /// assert_eq!(iter.next(), Some(&1));
    /// ```
    fn create_iterator<'a>(&'a self) -> Self::Iterator<'a>;
}

/// A factory for creating Vec collections and their iterators.
///
/// This struct implements the `CollectionFactory` trait for `Vec<T>`,
/// demonstrating how to use GATs with concrete types.
#[derive(Debug)]
pub struct VecFactory<T>(Vec<T>);

impl<T: Clone> CollectionFactory for VecFactory<T> {
    type Collection<'a>
        = Vec<T>
    where
        T: 'a;

    type Iterator<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    fn create_collection<'a>(&'a self) -> Self::Collection<'a> {
        self.0.clone()
    }

    fn create_iterator<'a>(&'a self) -> Self::Iterator<'a> {
        self.0.iter()
    }
}
