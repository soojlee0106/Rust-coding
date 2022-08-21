impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut matches = 0;

        while n > 1 {
            matches += n / 2;

            if n % 2 == 0 {
                n /= 2;
            } else {
                n /= 2;
                n += 1;
            }
        }

        matches
    }
}

//Minimum Number of Moves
impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        let mut moves = 0;

        seats.sort_unstable();
        students.sort_unstable();

        for (i, j) in seats.iter().zip(students.iter()) {
            moves += (i - j).abs();
        }

        moves
    }
}
