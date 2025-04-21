trait Container {
    type Item<'a>
    where
        Self: 'a;

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

pub fn main_gadts() {
    let vec = vec![1, 2, 3];
    let item = vec.get_item();
    assert_eq!(item, Some(&1));
    println!("{:?}", item);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_item() {
        let xs = vec![1, 2, 3];
        let item = xs.get_item();
        assert_eq!(item, Some(&1));
    }
}
