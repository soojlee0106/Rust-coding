use std::io;
use std::io::*;

fn main(){
    println!("Enter the first name of the person you want to write to:");

    let mut first_name = String::new();
    std::io::stdin().read_line(&mut first_name);
    first_name.pop();

    println!("Dear {},",first_name);
    println!("  I'm writing this letter because I miss you.");
    println!("Lots of things have happened since I started learning Rust.");
    println!("I started off with Python, and now I'm learning a completely new language.");

    println!("Enter the name of another close friend:");

    let mut friend_name = String::new();
    std::io::stdin().read_line(&mut friend_name);
    friend_name.pop();

    println!("Have you seen {} lately?",friend_name);

    println!("Enter m if the friend is male or f if the friend is female:");
    let mut friend_sex = String::new();
    std::io::stdin().read_line(&mut friend_name);

    match friend_sex.as_str() {
        "m" => {println!("If you see {} please ask him to call me.",friend_name);}
        "f"=> {println!("If you see {} please ask him to call me.",friend_name);}
        _=> {println!("If you see {} please ask them to call me.",friend_name);}
    }
}