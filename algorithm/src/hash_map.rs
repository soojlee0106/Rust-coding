use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[must_use]
pub fn price_book(item: &str) -> f64 {
    let mut prices = HashMap::new();
    prices.insert("apple", 0.67);
    prices.insert("milk", 1.49);
    prices.insert("avocado", 1.49);

    return *prices.get(item).unwrap();
}

#[must_use]
pub fn check_vote<'a>(voted: &mut HashMap<&'a str, bool>, name: &'a str) -> String {
    match voted.entry(name) {
        Entry::Occupied(_) => "Kick them out!".to_string(),
        Entry::Vacant(v) => {
            v.insert(true);
            "Let them vote!".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn apple_price() {
        assert_eq!(super::price_book("apple"), 0.67);
    }

    #[test]
    fn avo_price() {
        assert_eq!(super::price_book("avocado"), 1.49);
    }

    #[test]
    fn new_voter() {
        let mut voted = HashMap::new();
        voted.insert("David", true);
        assert_eq!(super::check_vote(&mut voted, "Soo"), "Let them vote!");
    }

    #[test]
    fn already_voted() {
        let mut voted = HashMap::new();
        voted.insert("Bly", true);

        assert_eq!(super::check_vote(&mut voted, "Bly"), "Kick them out!");
        assert_eq!(super::check_vote(&mut voted, "Soo"), "Let them vote!");
    }
}
