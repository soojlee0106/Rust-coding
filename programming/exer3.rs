fn main() {
    //1. Convert miles to kilometers
    println!("Enter the number of miles.");
    let mut miles = String::new();
    std::io::stdin().read_line(&mut miles).unwrap();
    let miles: f32 = miles.trim().parse().unwrap();
    let kilo = miles * 1.609;

    println!("{} miles is {} kilometers.", miles, kilo);

    //3. Compare input integer values
    println!("\nLet's compare numbers.");
    println!("Enter a number.");
    let mut firstnumber = String::new();
    std::io::stdin().read_line(&mut firstnumber).unwrap();
    let firstnumber: i32 = firstnumber.trim().parse().unwrap();

    println!("\nEnter a second number.");
    let mut secnumber = String::new();
    std::io::stdin().read_line(&mut secnumber).unwrap();
    let secnumber: i32 = secnumber.trim().parse().unwrap();

    println!(
        "This is the bigger number. {}",
        std::cmp::max(firstnumber, secnumber)
    );
    println!(
        "This is the smaller number. {}",
        std::cmp::min(firstnumber, secnumber)
    );
    println!(
        "This is the difference between the two numbers. {}",
        (firstnumber - secnumber)
    );
    println!(
        "This is the product of the two numbers. {}",
        (firstnumber * secnumber)
    );

    //11. Coin calculator
    println!("\nThis is a simple coin calculator.");
    println!("How many pennies do you have?");
    let mut pennies = String::new();
    std::io::stdin().read_line(&mut pennies).unwrap();
    let pennies: i32 = pennies.trim().parse().unwrap();

    println!("How many nickels do you have?");
    let mut nickels = String::new();
    std::io::stdin().read_line(&mut nickels).unwrap();
    let nickels: i32 = nickels.trim().parse().unwrap();

    println!("How many dimes do you have?");
    let mut dimes = String::new();
    std::io::stdin().read_line(&mut dimes).unwrap();
    let dimes: i32 = dimes.trim().parse().unwrap();

    println!("How many quarters do you have?");
    let mut quarters = String::new();
    std::io::stdin().read_line(&mut quarters).unwrap();
    let quarters: i32 = quarters.trim().parse().unwrap();

    println!("How many half dollars do you have?");
    let mut hd = String::new();
    std::io::stdin().read_line(&mut hd).unwrap();
    let hd: i32 = hd.trim().parse().unwrap();

    println!("How many dollars do you have?");
    let mut dollars = String::new();
    std::io::stdin().read_line(&mut dollars).unwrap();
    let dollars: i32 = dollars.trim().parse().unwrap();

    let value = pennies + nickels * 5 + dimes * 10 + quarters * 25 + hd * 50 + dollars * 100;

    println!(
        "\n You have {} pennies.
    \n You have {} dimes.
    \n You have {} quarters.
    \n You have {} half dollars.
    \n You have {} dollars.
    \n The value of all your coins is {} cents.",
        pennies, dimes, quarters, hd, dollars, value
    );
}
