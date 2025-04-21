pub trait Semigroup {
    fn combine(&self, other: &Self) -> Self;
}

pub trait Monoid: Semigroup + Clone {
    fn empty() -> Self;
}

// Implementing for built-in types
impl Semigroup for String {
    fn combine(&self, other: &Self) -> Self {
        let mut combined = self.clone();
        combined.push_str(other);
        combined
    }
}

impl Monoid for String {
    fn empty() -> Self {
        String::new()
    }
}

// Product and Sum types for numbers
#[derive(Clone)]
pub struct Product<T>(T);

impl From<u8> for Product<u8> {
    fn from(value: u8) -> Self {
        Product(value)
    }
}

#[derive(Clone)]
pub struct Sum<T>(T);

impl From<u8> for Sum<u8> {
    fn from(value: u8) -> Self {
        Sum(value)
    }
}

impl<T: Clone + std::ops::Mul<Output = T>> Semigroup for Product<T> {
    fn combine(&self, other: &Self) -> Self {
        Product(self.0.clone() * other.0.clone())
    }
}

impl<T: Clone + std::ops::Mul<Output = T> + From<u8>> Monoid for Product<T> {
    fn empty() -> Self {
        Product(T::from(1))
    }
}

impl<T: Clone + std::ops::Add<Output = T>> Semigroup for Sum<T> {
    fn combine(&self, other: &Self) -> Self {
        Sum(self.0.clone() + other.0.clone())
    }
}

impl<T: Clone + std::ops::Add<Output = T> + From<u8>> Monoid for Sum<T> {
    fn empty() -> Self {
        Sum(T::from(0))
    }
}

pub fn combine_all<M: Monoid>(values: &[M]) -> M {
    values
        .iter()
        .fold(M::empty(), |acc, value| acc.combine(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_strings() {
        let xs = ["Hello ", "typeclasses ", "in Rust"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let result = combine_all(&xs);
        assert_eq!(result, "Hello typeclasses in Rust");
    }

    #[test]
    fn combine_numbers() {
        let xs = [1, 2, 3, 4].into_iter().map(Sum::from).collect::<Vec<_>>();
        let result = combine_all(&xs);
        assert_eq!(result.0, 10);
    }

    #[test]
    fn combine_products() {
        let xs = [1, 2, 3, 4]
            .into_iter()
            .map(Product::from)
            .collect::<Vec<_>>();
        let result = combine_all(&xs);
        assert_eq!(result.0, 24);
    }
}
