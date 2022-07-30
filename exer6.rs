//10. Permutation Combination
use std::process::exit;

fn main() {
    pub fn factorial(num: u128) -> u128 {
        let mut prod: u128 = 1;
        for i in 1..num {
            prod = if let Some(p) = prod.checked_mul(i) {
                p
            } else {
                eprintln!("The number is too big for this calculator to handle. Try again with different numbers.");
                exit(1);
            }
        }
        prod
    }

    println!("Welcome to the permutation or combination calculator.");
    println!("What would you like to calculate? Enter P for permutation, and C for combination.");

    let mut percom = String::new();
    std::io::stdin().read_line(&mut percom).unwrap();
    let percom = percom.trim();

    match percom {
        "P" => {
            println!("Please enter your a and b values. Start with a.");
            let mut a = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut a) {
                eprintln!("This is not a valid a value. Try again.{}", e);
                exit(1);
            };

            let a: u128 = if let Ok(a) = a.trim().parse() {
                a
            } else {
                eprintln!("This is not a valid a value. Try again.");
                exit(1);
            };

            if a == 0 {
                eprintln!("The factorial of 0 is not valid. Try again.");
                exit(1);
            }

            println!("Now enter your b value.");
            let mut b = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut b) {
                eprintln!("This is not a valid b value. Try again.{}", e);
                exit(1);
            };

            let b: u128 = if let Ok(b) = b.trim().parse() {
                b
            } else {
                eprintln!("This is not a valid b value. Try again.");
                exit(1);
            };

            if b == 0 {
                eprintln!("The factorial of 0 is not valid. Try again.");
                exit(1);
            }

            let afac = factorial(a);
            let abfac = factorial(a - b);
            let permutation = afac / abfac;

            println!("The permutation of a and b is {}.", permutation);
        }

        "C" => {
            println!("Please enter your a and b values.");
            println!("Please enter your a and b values. Start with a.");
            let mut a = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut a) {
                eprintln!("This is not a valid a value. Try again.{}", e);
                exit(1);
            };

            let a: u128 = if let Ok(a) = a.trim().parse() {
                a
            } else {
                eprintln!("This is not a valid a value. Try again.");
                exit(1);
            };

            if a == 0 {
                eprintln!("The factorial of 0 is not valid. Try again.");
                exit(1);
            }

            println!("Now enter your b value.");
            let mut b = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut b) {
                eprintln!("This is not a valid b value. Try again.{}", e);
                exit(1);
            };

            let b: u128 = if let Ok(b) = b.trim().parse() {
                b
            } else {
                eprintln!("This is not a valid b value. Try again.");
                exit(1);
            };

            if b == 0 {
                eprintln!("The factorial of 0 is not valid. Try again.");
                exit(1);
            }

            let afac = factorial(a);
            let abfac = factorial(a - b);

            let permutation = afac / abfac;

            let bfac = factorial(b);

            let combination = permutation / bfac;
            println!("The combination of a and b is {}.", combination);
        }

        _ => {
            eprintln!("Not a valid choice. Try again with either P or C.");
            exit(1)
        }
    };
}
