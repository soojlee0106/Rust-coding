#[allow(dead_code)]
fn get_pivot_index(array: &mut [usize], start: usize, end: usize) -> usize {
    let pivot = array[end];
    let mut pivot_index = start;
    let mut i = start;

    while i < end {
        if array[i] < pivot {
            array.swap(i, pivot_index);
            pivot_index += 1;
        }

        i += 1;
    }
    array.swap(pivot_index, end);
    pivot_index
}

#[allow(dead_code)]
fn quick_sort(array: &mut [usize], start: usize, end: usize) -> &mut [usize] {
    if start < end {
        let pivot = get_pivot_index(array, start, end);
        quick_sort(array, start, pivot - 1);
        quick_sort(array, pivot + 1, end);
        array
    } else {
        array
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn quicksort_empty() {
        let mut array = [];
        let test = super::quick_sort(&mut array, 0, 0);
        assert_eq!(test, vec![]);
    }

    #[test]
    fn quicksort_five() {
        let mut array = [0, 5, 100, 9, 70];
        let test = super::quick_sort(&mut array, 0, 4);
        assert_eq!(test, vec![0, 5, 9, 70, 100]);
    }

    #[test]
    fn quicksort_many() {
        let mut array = [3, 7, 1, 2, 89, 36, 55, 34];
        let test = super::quick_sort(&mut array, 0, 7);
        assert_eq!(test, vec![1, 2, 3, 7, 34, 36, 55, 89]);
    }
}
