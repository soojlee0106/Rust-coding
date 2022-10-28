#[allow(dead_code)]
fn greedy_knapsack(mut knapsack_capacity: usize, item_sizes: &[usize]) -> (usize, Vec<usize>) {
    //Solution for knapsack problem for greedy algorithms
    let mut max_item_numbers = 0; //how many items you can fit
    let mut filled_knapsack: Vec<usize> = vec![]; //how many of each item you fit in

    if knapsack_capacity == 0 || item_sizes.is_empty() {
        return (0, vec![]);
    }

    for item in item_sizes {
        max_item_numbers += knapsack_capacity / *item;
        filled_knapsack.push(knapsack_capacity / *item);
        knapsack_capacity %= *item;
    }

    (max_item_numbers, filled_knapsack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_knapsack() {
        let test = greedy_knapsack(0, &[]);
        let answer = (0, vec![]);
        assert_eq!(test, answer);
    }

    #[test]
    fn simple_knapsack() {
        let test = greedy_knapsack(1260, &[500, 100, 50, 10]);
        let answer = (6, vec![2, 2, 1, 1]); //2 of 500 size items, 2 of 100 size items, 1 of 50 size item, 1 of 10 size item
        assert_eq!(test, answer);
    }

    #[test]
    fn complex_knapsack() {
        let test = greedy_knapsack(3052340, &[2540, 1450, 730, 450, 240, 150, 70, 34, 21, 10]);
        let answer = (1205, vec![1201, 1, 0, 0, 1, 0, 1, 1, 0, 0]);
        assert_eq!(test, answer);
    }
}
