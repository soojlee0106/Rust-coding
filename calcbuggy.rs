use std::io;
use std::process::exit;

type CalcInt = i64;

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
    const BASE: CalcInt = 10;

    let mut val: CalcInt = 0;
    loop {
        let digit: CalcInt;
        match token(ps) {
            '0' => digit = 0,
            '1' => digit = 1,
            '2' => digit = 2,
            '3' => digit = 3,
            '4' => digit = 4,
            '5' => digit = 5,
            '6' => digit = 6,
            '7' => digit = 7,
            '8' => digit = 8,
            '9' => digit = 9,
            _ => break,
        };

        val = val * BASE + digit;
        ps.index += 1;
    }

    val
}

fn factor(ps: &mut ParseState) -> CalcInt {
    let value: CalcInt;

    if token(ps) == '(' {
        lex_match(ps, '(');
        value = expr(ps);
        lex_match(ps, ')');
    } else if token(ps).is_ascii_digit() || token(ps) == '+' || token(ps) == '-' {
        value = scan_digits(ps);
    } else {
        exit(1);
    }

    value
}

fn term(ps: &mut ParseState) -> CalcInt {
    let mut value: CalcInt = factor(ps);

    while token(ps) == '*' || token(ps) == '/' {
        match token(ps) {
            '*' => {
                lex_match(ps, '*');

                value *= factor(ps);
            }

            '/' => {
                lex_match(ps, '/');

                value /= factor(ps);
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
    \nYou can enter x to quit."
    );

    loop {
        println!("Write your expression here.\t");

        let mut ps = ParseState {
            line: String::new(),
            index: 1,
        };

        if ps.line == "x" {
            break;
        }

        match io::stdin().read_line(&mut ps.line) {
            Ok(_n) => {
                let result = expr(&mut ps);

                println!("Your result is: {}", result);
            }

            Err(error) => eprintln!("There's an error. Try again. {}", error),
        }
    }
}
