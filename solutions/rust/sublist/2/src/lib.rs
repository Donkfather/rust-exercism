use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_subset_of<T: PartialEq>(a: &[T], b: &[T], reverse: bool) -> Comparison {
    use Comparison::*;
    let a_first = a.first().unwrap();
    // Get all the indexes in b that correspond to a `a` element
    for (i, bx) in b.iter().enumerate() {
        if bx == a_first && a.eq(&b[i..i + a.len()]) {
            return if reverse { Superlist } else { Sublist };
        }
    }

    Unequal
}
pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match first.len().cmp(&second.len()) {
        Ordering::Equal => {
            if first == second {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if first.is_empty() {
                return Comparison::Sublist;
            }
            is_subset_of(first, second, false)
        }
        Ordering::Greater => {
            if second.is_empty() {
                return Comparison::Superlist;
            }
            is_subset_of(second, first, true)
        }
    }
}