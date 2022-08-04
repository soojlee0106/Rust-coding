//Richest Customer Wealth
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let sumWealth = accounts.iter().map(|i| i.iter().sum());
    let maxWealth = sumWealth.max().unwrap();
    return maxWealth;
}

//Shuffle the Array
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut v = vec![0; nums.len()];
    let half = n as usize;

    for i in 0..half {
        v[2 * i] = nums[i];
        v[2 * i + 1] = nums[i + half];
    }
    v
}
