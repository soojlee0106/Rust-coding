//Minimum Sum of Four Digit Number
pub fn minimum_sum(mut num: i32) -> i32 {
    let mut fullvec = Vec::with_capacity(4);

    (0..4).for_each(|_| {
        fullvec.push(num % 10);
        num /= 10;
    });

    fullvec.sort();

    if let &[a1, a2, a3, a4] = fullvec.as_slice() {
        return 10 * a1 + 10 * a2 + a3 + a4;
    
    unreachable!()
    }
}

// Number of Good Pairs
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut pairs= 0;
    
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i]==nums[j] {
            pairs+=1;
            }
        }
    } pairs
}

//Kids with the Greatest Number of Candies
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let &greatCandy= candies.iter().max().unwrap();
    candies.iter().map(|x|x+extra_candies>=greatCandy).collect()
}

