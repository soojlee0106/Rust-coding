use std::process::exit;
use std::vec::Vec;

fn main() {
    let mut vec = Vec::new();
    println!("\nLet's create a vector.");

    while true {
        println!("Enter a number.");
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).unwrap();

        let number: i32 = if let Ok(number) = number.trim().parse() {
            number
        } else {
            let min_value = *vec.iter().min().unwrap();
            let max_value = *vec.iter().max().unwrap();
            let elements = vec.len();
            let sum: i32 = vec.iter().sum();

            println!("The smallest so far: {} ", min_value);
            println!("The largest so far: {}", max_value);
            println!("The number of values: {}", elements);
            println!("The sum of values: {}", sum);
            println!("The entire vector: {:?}", vec);
            exit(1);
        };

        println!("\nWhat is the unit?");
        let mut unit = String::new();
        std::io::stdin().read_line(&mut unit).unwrap();
        unit.pop();

        match unit.as_str() {
            "cm" | "m" | "in" | "ft" => {
                println!("Acceptable unit. Value is {}{}", number, unit);
            }
            _ => {
                println!("Unacceptable unit.");
            }
        };

        if unit.as_str() == "m" {
            let convnumber = number as i32 * 100;
            vec.push(convnumber as i32);
        }
        if unit.as_str() == "in" {
            let convnumber = (number as f64) * 2.54;
            vec.push(convnumber as i32);
        }
        if unit.as_str() == "ft" {
            let convnumber = (number as f64) * 2.54 * 12.0;
            vec.push(convnumber as i32);
        }

        vec.sort_unstable();
    }
}
