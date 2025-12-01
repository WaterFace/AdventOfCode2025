use std::cmp::Ordering;

pub trait SortedExt<T: std::cmp::Ord> {
    fn sorted(self) -> Self
    where
        Self: Sized,
    {
        self.sorted_by(<T as Ord>::cmp)
    }

    fn sorted_by<F>(self, f: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering;

    fn sorted_by_key<F, K>(self, f: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

impl<T: std::cmp::Ord> SortedExt<T> for Vec<T> {
    fn sorted_by<F>(mut self, f: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.sort_by(f);
        self
    }

    fn sorted_by_key<F, K>(mut self, f: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.sort_by_key(f);
        self
    }
}
