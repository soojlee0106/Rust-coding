//Exercises in Chapter 7 (improving the calculator more)
//Add q,h options. Improve error message.
//Now supports pi and e.
//Now supports % calculation.

use std::io;
use std::process::exit;

type CalcInt = f64;

struct ParseState {
    line: String,
    index: usize,
}

fn token(ps: &ParseState) -> char {
    ps.line.as_bytes()[ps.index - 1] as char
}

fn lex_match(ps: &mut ParseState, expected: char) {
    if token(ps) == expected {
        ps.index += 1;
    }
}

fn scan_digits(ps: &mut ParseState) -> CalcInt {
    const BASE: CalcInt = 10.0;

    let mut val: CalcInt = 0.0;
    loop {
        let digit: CalcInt;
        match token(ps) {
            '0' => digit = 0.0,
            '1' => digit = 1.0,
            '2' => digit = 2.0,
            '3' => digit = 3.0,
            '4' => digit = 4.0,
            '5' => digit = 5.0,
            '6' => digit = 6.0,
            '7' => digit = 7.0,
            '8' => digit = 8.0,
            '9' => digit = 9.0,
            'π' => digit = 3.1,
            'e' => digit = 2.7,
            _ => break,
        };

        val = val * BASE + digit;
        ps.index += 1;
    }

    val
}

fn factor(ps: &mut ParseState) -> CalcInt {
    let value: CalcInt;

    if token(ps) == 'q' {
        exit(1);
    }

    if token(ps) == 'h' || token(ps) == 'H' {
        println!(
            "This calculator works with +,-,*,/,√,^,%. Other signs are not supported. 
        \nAlso note that π is simplified as 3.1 and e is simplified as 2.7.
        \nType q to quit."
        );
    }

    if token(ps) == '(' {
        lex_match(ps, '(');
        value = expr(ps);
        lex_match(ps, ')');
    } else if token(ps).is_ascii_digit() || token(ps) == '+' || token(ps) == '-' {
        value = scan_digits(ps);
    } else {
        eprintln!("The calculator will start over.");
        exit(1);
    }

    value
}

fn term(ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = factor(ps);

    while token(ps) == '*' || token(ps) == '/' || token(ps) == '√' || token(ps) == '^' {
        match token(ps) {
            '*' => {
                lex_match(ps, '*');
                value *= factor(ps);
            }

            '/' => {
                lex_match(ps, '/');
                value /= factor(ps);
            }

            '√' => {
                lex_match(ps, '√');
                value = factor(ps).sqrt();
                if factor(ps) < 0.0 {
                    eprintln!("This calculator can't handle negative roots. Try again.");
                    exit(1);
                }
            }

            '^' => {
                lex_match(ps, '^');
                value = value.powf(factor(ps));
            }

            '%' => {
                lex_match(ps, '%');
                value /= factor(ps) * 100.0;
            }

            _ => {}
        }
    }

    value
}

fn expr(ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = term(ps);

    match token(ps) {
        '+' => {
            lex_match(ps, '+');
            value += term(ps);
        }
        '-' => {
            lex_match(ps, '-');
            value -= term(ps);
        }

        _ => {}
    }

    value
}

fn main() {
    println!(
        "Welcome to our simple calculator. 
    \nPlease enter expressions using floating-point numbers.
    \nWe can handle addition, subtraction, multiplication and division. 
    \nWe upgraded the calculator so you can use squareroot(√),power(^) and percentage(%) too.
    \nPlease note that π is simplified as 3.1 and e is simplified as 2.7.
    \nType q to quit, and h or H to see helpful info."
    );

    loop {
        println!("Write your expression here. Press enter afterwards.\t");

        let mut ps = ParseState {
            line: String::new(),
            index: 1,
        };

        match io::stdin().read_line(&mut ps.line) {
            Ok(_n) => {
                let result = expr(&mut ps);
                std::io::stdin().read_line(&mut ps.line).unwrap();
                println!("Your result is: {}", result);
            }

            Err(error) => {
                eprintln!("There's an error with the number or math sign you entered. Press h to see help. Try again. {}", error);
                exit(1);
            }
        }
    }
}
