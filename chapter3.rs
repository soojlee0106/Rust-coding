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
    let number: i32 = age.trim().parse().unwrap();
    age.pop();

    if number <= 0 || number >= 110 {
        println!("you're kidding!");
    } else {
        println!(
            "I heard you just had a birthday and you are {} years old.",
            number
        );

        if number < 12 {
            println!("Next year you will be {}.", number + 1);
        } else if number == 17 {
            println!("Next year you will be able to vote.")
        } else if number > 70 {
            println!("I hope you are enjoying retirement.")
        } else {
            println!("")
        };
    };

    println!("\nI hope you have a great rest of the week and I hope to hear from you soon.");
    println!("Sincerely, {}.", first_name);
}
