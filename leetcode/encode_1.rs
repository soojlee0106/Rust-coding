//Decode XORed Array
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut original = vec![first];
        for i in 0..encoded.len() {
            original.push(original[i] ^ encoded[i])
        }
        original
    }
}

//Decompress Run-Length Encoded List
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut decompressed = Vec::new();
        let mut freq = 0;

        for (i, j) in nums.iter().enumerate() {
            if i % 2 == 0 {
                freq = *j;
            } else {
                for n in 0..freq {
                    decompressed.push(*j);
                }
            }
        }
        decompressed
    }
}

//Shuffle String
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut shuffled = vec![' '; indices.len()];
        for (i, j) in indices.iter().enumerate() {
            shuffled[*j as usize] = s.as_bytes()[i] as char;
        }
        shuffled.iter().collect::<String>()
    }
}
