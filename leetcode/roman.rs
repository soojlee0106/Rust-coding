pub fn roman_to_int(s: &str) -> i32 {
    let values = s
        .chars()
        .map(|x| match x {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        })
        .collect::<Vec<i32>>();
    let mut sum = values[0];
    for i in 0..values.len() - 1 {
        sum += if values[i] >= values[i + 1] {
            values[i + 1]
        } else {
            values[i + 1] - (values[i] * 2)
        };
    }
    sum
}

fn main() {
    println!("Enter the roman numeral. This will convert it to arabic numerals.");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.pop();

    println!("The result is {}", roman_to_int(&s));
}
