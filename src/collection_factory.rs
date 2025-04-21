pub trait CollectionFactory {
    type Collection<'a>
    where
        Self: 'a;
    type Iterator<'a>
    where
        Self: 'a;

    fn create_collection<'a>(&'a self) -> Self::Collection<'a>;
    fn create_iterator<'a>(&'a self) -> Self::Iterator<'a>;
}

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
