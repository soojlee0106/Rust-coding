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

    println!("Have you seen {} lately?",first_name);
 }