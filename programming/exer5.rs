use std::process::exit;

//2. Kelvin Changer
fn main() {
    println!("This changes the given Celcius to Kelvin.");
    println!("Enter the Celcius value.");
    let mut celc = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut celc) {
        eprintln!("This is not a valid Celcius value. Try again.{}", e);
        exit(1);
    }

    let celc: f64 = if let Ok(celc) = celc.trim().parse() {
        celc
    } else {
        eprintln!("This is not a valid Celcius value. Try again.");
        exit(1);
    };

    let kelv = celc + 273.15;

    if celc < -273.15 {
        eprintln!("This is not a valid temperature. Enter a value larger than -273.15.");
    } else {
        println!("The Celcius is {} Kelvins.", kelv);
    }

    //7. Quadratic Formula
    println!("\nThis is the quadratic formula calculator.");
    println!("The quadratic formula takes the form of a*x^2+b*2+c=0.");
    println!("Enter a,b and c to calculate x values (the root).");
    println!("Enter the a value.");
    let mut a = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut a) {
        eprintln!("This is not a valid value of a. Try again. {}", e);
        exit(1);
    }

    let a: i32 = if let Ok(a) = a.trim().parse() {
        a
    } else {
        eprintln!("This is not a valid value of a. Try again.");
        exit(1);
    };

    println!("Enter the b value.");
    let mut b = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut b) {
        eprintln!("This is not a valid value of b. Try again. {}", e);
        exit(1);
    }

    let b: i32 = if let Ok(b) = b.trim().parse() {
        b
    } else {
        eprintln!("This is not a valid value of b. Try again.");
        exit(1);
    };

    println!("Enter the c value.");
    let mut c = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut c) {
        eprintln!("This is not a valid value of c. Try again. {}", e);
        exit(1);
    }
    let c: i32 = if let Ok(c) = c.trim().parse() {
        c
    } else {
        eprintln!("This is not a valid value of b. Try again.");
        exit(1);
    };

    let root = f64::from(b.pow(2) - (4 * a * c));

    let x1 = (f64::from(-b) + root.sqrt()) / f64::from(2 * a);
    let x2 = (f64::from(-b) - root.sqrt()) / f64::from(2 * a);

    if root < 0.0 {
        eprintln!("These are not  valid values for the formula. Change your values.");
        exit(1);
    } else {
        println!("The root values are {} and {}.", x1, x2);
    }
}
