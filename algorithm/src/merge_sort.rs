#[allow(dead_code)]
fn merge_sort(array: Vec<usize>) -> Vec<usize> {
    if array.len() < 2 {
        //already sorted
        return array;
    };

    let mid_index = array.len() / 2;
    let lower_array = merge_sort(array[0..mid_index].to_vec());
    let higher_array = merge_sort(array[mid_index..array.len()].to_vec());

    let mut merged_array: Vec<usize> = vec![];
    let mut low_index = 0;
    let mut high_index = 0;

    while low_index < lower_array.len() && high_index < higher_array.len() {
        if lower_array[low_index] < higher_array[high_index] {
            merged_array.push(lower_array[low_index]);
            low_index += 1;
        } else {
            merged_array.push(higher_array[high_index]);
            high_index += 1;
        }
    }
    merged_array.append(&mut lower_array[low_index..lower_array.len()].to_vec());
    merged_array.append(&mut higher_array[high_index..higher_array.len()].to_vec());
    merged_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_empty() {
        let array = vec![];
        let test = merge_sort(array);
        let expected: Vec<usize> = vec![];
        assert_eq!(test, expected);
    }

    #[test]
    fn merge_simple() {
        let array = vec![5, 3, 1];
        let test = merge_sort(array);
        let expected: Vec<usize> = vec![1, 3, 5];
        assert_eq!(test, expected);
    }

    #[test]
    fn merge_complex() {
        let array = vec![5, 9, 3, 8, 2, 1, 4, 7, 6];
        let test = merge_sort(array);
        let expected: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(test, expected);
    }

    #[test]
    fn merge_complex_large() {
        let array = vec![56, 29, 154, 721, 654, 88, 36, 274, 36, 199, 97];
        let test = merge_sort(array);
        let expected: Vec<usize> = vec![29, 36, 36, 56, 88, 97, 154, 199, 274, 654, 721];
        assert_eq!(test, expected);
    }
}
