use std::process::exit;

#[allow(dead_code)]
#[must_use]
fn countdown(i: i32) -> i32 {
    if i <= 0 {
        0
    } else {
        countdown(i - 1)
    }
}

#[allow(dead_code)]
#[must_use]
fn factorial(i: i32) -> i32 {
    if i == 1 {
        1
    } else if i <= 0 {
        eprintln!("Numbers equal to or smaller than 0 are not supported.");
        exit(1);
    } else {
        i * factorial(i - 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn countdown_with_zero() {
        assert_eq!(super::countdown(0), 0);
    }

    #[test]
    fn countdown_with_five() {
        assert_eq!(super::countdown(5), 0);
    }

    #[test]
    fn factorial_with_one() {
        assert_eq!(super::factorial(1), 1);
    }

    #[test]
    fn factorial_with_five() {
        assert_eq!(super::factorial(5), 120);
    }
}
