use std::collections::VecDeque;

#[allow(dead_code)]
fn partition(array: &mut [usize], start_index: usize, end_index: usize) -> usize {
    let pivot = array[end_index];
    let mut pivot_index = start_index;
    let mut i = start_index;

    while i < end_index {
        if array[i] < pivot {
            array.swap(i, pivot_index);
            pivot_index += 1;
        }

        i += 1;
    }
    array.swap(pivot_index, end_index);
    pivot_index
}

#[allow(dead_code)]
fn quick_sort(array: &mut [usize]) -> &mut [usize] {
    if array.len() < 2 {
        return array; //no need to be sorted
    }

    let mut stack = VecDeque::new();
    let mut start_index = 0;
    let mut end_index = array.len() - 1;

    stack.push_back((start_index, end_index));

    while !stack.is_empty() {
        (start_index, end_index) = stack.pop_back().unwrap();
        let pivot_index = partition(array, start_index, end_index);
        if pivot_index - 1 > start_index {
            stack.push_back((start_index, pivot_index - 1));
        }
        if pivot_index + 1 < end_index {
            stack.push_back((pivot_index + 1, end_index));
        }
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_empty() {
        let mut array = [];
        let test = quick_sort(&mut array);
        assert_eq!(test, vec![]);
    }

    #[test]
    fn quicksort_five() {
        let mut array = [0, 5, 100, 9, 70];
        let test = quick_sort(&mut array);
        assert_eq!(test, vec![0, 5, 9, 70, 100]);
    }

    #[test]
    fn quicksort_many() {
        let mut array = [3, 7, 1, 2, 89, 36, 55, 34];
        let test = quick_sort(&mut array);
        assert_eq!(test, vec![1, 2, 3, 7, 34, 36, 55, 89]);
    }
}
