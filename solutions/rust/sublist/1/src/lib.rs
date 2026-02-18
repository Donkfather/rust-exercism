#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub trait SubsetOf<T> {
    fn is_subset_of(&self, other: T) -> Comparison;
    fn check_empty_or_equal(&self, other: T) -> Option<Comparison>;
}

impl SubsetOf<&[i32]> for &[i32] {
    fn is_subset_of(&self, other: &[i32]) -> Comparison {
        if let Some(empty_or_equal) = self.check_empty_or_equal(other) {
            return empty_or_equal;
        }
        if self.len() > other.len() {
            return match other.is_subset_of(self) {
                Comparison::Equal => Comparison::Equal,
                Comparison::Sublist => Comparison::Superlist,
                _ => Comparison::Unequal,
            };
        }

        // Get all the indexes in other that correspond to self first element
        let mut found: Vec<(usize, usize)> = Vec::new();
        for i in 0..other.len() {
            if other[i] == self[0] {
                found.push((i, i + self.len() - 1));
            }
        }

        if found.len() == 0 {
            return Comparison::Unequal;
        };

        for (start, end) in found {
            if self.eq(&&other[start..=end]) {
                return Comparison::Sublist;
            }
        }

        Comparison::Unequal
    }

    fn check_empty_or_equal(&self, other: &[i32]) -> Option<Comparison> {
        if self.len() == other.len() {
            return match self.eq(&other) {
                true => Some(Comparison::Equal),
                false => Some(Comparison::Unequal),
            };
        }

        if self.is_empty() {
            return Some(Comparison::Sublist);
        }

        if other.is_empty() {
            return Some(Comparison::Superlist);
        }

        None
    }
}

pub fn sublist(first: &[i32], second: &[i32]) -> Comparison {
    first.is_subset_of(second)
}
