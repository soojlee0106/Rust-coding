use std::i32::MAX;
use std::i32::MIN;

fn main() {
    println!("Enter the first name of the person you want to write to:");

    let mut first_name = String::new();
    std::io::stdin().read_line(&mut first_name).unwrap();
    first_name.pop();

    println!("Dear {},", first_name);
    println!("  I'm writing this letter because I miss you.");
    println!("Lots of things have happened since I started learning Rust.");
    println!("I started off with Python, and now I'm learning a completely new language.");

    println!("\nEnter the name of another close friend:");

    let mut friend_name = String::new();
    std::io::stdin().read_line(&mut friend_name).unwrap();
    friend_name.pop();

    println!("Have you seen {} lately?", friend_name);

    println!("\nEnter m if the friend is male or f if the friend is female:");
    let mut friend_sex = String::new();
    std::io::stdin().read_line(&mut friend_sex).unwrap();
    friend_sex.pop();

    match friend_sex.as_str() {
        "m" => {
            println!("If you see {} please ask him to call me.", friend_name);
        }
        "f" => {
            println!("If you see {} please ask her to call me.", friend_name);
        }
        _ => {
            println!("If you see {} please ask them to call me.", friend_name);
        }
    }

    println!("\nEnter the age of your friend:");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age: i32 = age.trim().parse().unwrap();

    assert_eq!(i32::MAX, 2_147_483_647);
    assert_eq!(i32::MIN, -2_147_483_648);

    match age {
        MIN..=0 | 110..=MAX => {
            println!("you're kidding!");
        }
        _ => {
            println!(
                "I heard you just had a birthday and you are {} years old.",
                age
            );

            match age {
                1..=11 => {
                    println!("Next year you will be {}.", age + 1);
                }
                17 => {
                    println!("Next year you will be able to vote.");
                }
                70..=109 => {
                    println!("I hope you are enjoying retirement.");
                }
                _ => {
                    println!();
                }
            }
        }
    }

    println!(
        "\nI hope you have a great rest of the week and I hope to hear from you soon.
    \nYours sincerely,
    \n
    \n{}",
        first_name
    );
}
