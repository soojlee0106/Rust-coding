//Binary Search
#[must_use]
pub fn binary_search(list: Vec<i32>, item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = list[mid];

        if guess == item {
            return Some(mid);
        } else if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}
