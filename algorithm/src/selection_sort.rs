#[must_use]
pub fn find_smallest(array: &[usize]) -> usize {
    let mut smallest = array[0];
    let mut smallest_index = 0;

    for item in array.iter().enumerate().skip(1) {
        if array[item.0] < smallest {
            smallest = array[item.0];
            smallest_index = item.0;
        }
    }
    smallest_index
}

#[must_use]
pub fn selection_sort(mut array: Vec<usize>) -> Vec<usize> {
    for i in 0..array.len() {
        let smallest_index = find_smallest(&array[i..]);
        let smallest = array[smallest_index + i];
        let tmp = array[i];
        array[i] = smallest;
        array[smallest_index + i] = tmp;
    }
    array
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_smallest_one() {
        let array = vec![1];
        assert_eq!(super::find_smallest(&array), 0);
    }

    #[test]
    fn selection_sort_empty() {
        let array = vec![];
        let sorted_array = super::selection_sort(array);
        assert_eq!(sorted_array, vec![]);
    }

    #[test]
    fn selection_sort_one() {
        let array = vec![1];
        let sorted_array = super::selection_sort(array);
        assert_eq!(sorted_array, vec![1]);
    }

    #[test]
    fn selection_sort() {
        let array = vec![5, 3, 6, 2, 10];
        let sorted_array = super::selection_sort(array);
        assert_eq!(sorted_array, vec![2, 3, 5, 6, 10]);
    }
}
