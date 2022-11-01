use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

//Binary Search
#[must_use]
pub fn binary_search(list: &[usize], item: usize) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = list[mid];

        match guess.cmp(&item) {
            Less => low = mid + 1,
            Equal => return Some(mid),
            Greater => high = mid - 1,
        }
    }

    None
}
