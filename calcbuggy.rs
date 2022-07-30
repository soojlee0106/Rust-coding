use std::io;
use std::process::exit;

struct TokenStream {
    line: String,
    index: usize,
}

impl TokenStream {
    pub fn expr(&mut self) -> f64 {
        let mut left: f64 = self.term();
        let mut t = self.get();
        loop {
            match t {
                Token::Addition => {
                    let right = self.term();
                    left += right;
                    t = self.get();
                }
                Token::Subtraction => {
                    left -= self.term();
                    t = self.get();
                }
                _ => {
                    let _ = self.index - 1;
                    return left;
                }
            }
        }
    }

    fn get(&mut self) -> Token {
        if self.line.len() > self.index {
            match self.line.as_bytes()[self.index - 1] as char {
                ';' => {
                    let result = self.expr();
                    println!("Your result is: {}", result);
                    Token::Print
                }
                'q' => {
                    exit(1);
                }
                '(' => {
                    self.index += 1;
                    Token::LeftParenthesis
                }
                ')' => {
                    self.index += 1;
                    Token::RightParenthesis
                }
                '+' => {
                    self.index += 1;
                    Token::Addition
                }
                '-' => {
                    self.index += 1;
                    Token::Subtraction
                }
                '*' => {
                    self.index += 1;
                    Token::Multiplication
                }
                '/' => {
                    self.index += 1;
                    Token::Division
                }
                '.' | '0'..='9' => {
                    let bytes = self.line.as_bytes(); 
                    let len = read_f64(&bytes[self.index - 1..]); 
                    let s = unsafe { String::from_utf8_unchecked(bytes[self.index - 1..self.index - 1 + len].to_vec()) }; 
                    let val: f64 = s.parse().unwrap(); 
                    self.index += len;
                    Token::Number(val)
                }
                _ => {
                    eprintln!("This calculator can't handle such symbol. Try again.");
                    exit(1);
                }
            }
        } else {
            Token::Print
        }
    }

    fn primary(&mut self) -> f64 {
        let mut t = self.get(); 

        match t {
            Token::LeftParenthesis => {
                let d = self.expr();
                t = self.get();
                if t != Token::RightParenthesis {
                    eprintln!("')' expected.");
                    exit(1);
                }
                d
            }
            Token::Number(val) => val,
            _ => {
                eprintln!("No numbers or other valid characters were given. Try again.");
                exit(1);
            }
        }
    }

    fn term(&mut self) -> f64 {
        let mut left = self.primary(); 
        let mut t = self.get(); 

        loop {
            match t {
                Token::Multiplication => {
                    let right = self.primary(); 
                    left *= right;
                    t = self.get(); 
                }

                Token::Division => {
                    let right = self.primary();
                    if right == 0.0 {
                        eprintln!("O cannot be the denominator.");
                        exit(1);
                    }
                    left /= right;
                    t = self.get();
                }
                _ => {
                    self.index -= 1;
                    return left;
                }
            }
        }
    }
}

fn read_f64(s: &[u8]) -> usize {
    for (i, item) in s.iter().enumerate() {
        match item {
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'.' => {
                continue;
            }
            _ => {
                return i;
            }
        }
    }
    s.len()
}

#[derive(Debug, PartialEq)]
enum Token {
    LeftParenthesis,
    RightParenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Number(f64),
    Print,
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

        let mut toks = TokenStream {
            line: String::new(),
            index: 1,
        };

        match io::stdin().read_line(&mut toks.line) {
            Ok(_n) => {
                let result = toks.expr();
                println!("Your result is: {}", result);
            }

            Err(error) => {
                eprintln!("There's an error. Try again. {}", error);
                exit(1);
            }
        }
    }
}
