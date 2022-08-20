//Final Value of Variable
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;

        for op in operations {
            match op.as_str() {
                "--X" | "X--" => x -= 1,
                "++X" | "X++" => x += 1,
                _ => (),
            }
        }

        x
    }
}

//Number of Steps
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut count = 0;

        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            count += 1;
        }

        count
    }
}

//Create Target Array
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![];

        for i in 0..nums.len() {
            arr.insert(index[i] as usize, nums[i])
        }

        arr
    }
}
