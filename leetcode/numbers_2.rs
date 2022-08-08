//Subtract Sum from Product
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut sum: i32 = n % 10;
    let mut product: i32 = n % 10;

    while n / 10 != 0 {
        n /= 10;
        sum += n % 10;
        product *= n % 10;
    }

    product - sum
}

//How many are smaller?
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|i| nums.iter().filter(|j| j < &i).count() as i32)
        .collect()
}
