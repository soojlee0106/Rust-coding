use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug)]
struct Queue<T> {
    //create Queue struct
    pub deq: VecDeque<T>,
}

#[allow(dead_code)]
impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            deq: VecDeque::new(), //Double-ended queue for breadth-first search
        }
    }

    pub fn enqueue(&mut self, node: T) {
        self.deq.push_back(node); //append at the end of dequeue
    }

    pub fn dequeue(&mut self) -> T {
        self.deq.pop_front().unwrap() //remove from the front of dequeue
    }

    pub fn empty_queue(&self) -> bool {
        self.deq.is_empty()
    }
}

#[allow(dead_code)]
fn bfs(hashmap: &HashMap<&str, Vec<&str>>, name: &str) -> bool {
    let mut search_queue = Queue::new();
    let mut searched: HashMap<&str, bool> = HashMap::new();
    search_queue.enqueue(name);

    while !search_queue.empty_queue() {
        let person: &str = search_queue.dequeue();
        if let Entry::Vacant(e) = searched.entry(person) {
            if person == "seller" {
                return true;
            }
            let person_value = match hashmap.get(person) {
                Some(person_value) => person_value,
                None => continue,
            };
            for child in person_value {
                search_queue.enqueue(child);
            }
            e.insert(true);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_hashmap() {
        let hashmap = HashMap::new();
        let test = bfs(&hashmap, "Soo");
        let expected = false;
        assert_eq!(test, expected);
    }

    #[test]
    fn with_seller_simple() {
        let mut hashmap = HashMap::new();
        hashmap.insert("Soo", vec!["Bob", "Sally"]);
        hashmap.insert("Bob", vec!["Nao", "David"]);
        hashmap.insert("Sally", vec!["Joe", "seller"]);

        let test = bfs(&hashmap, "Soo");
        let expected = true;
        assert_eq!(test, expected);
    }

    #[test]
    fn with_seller_complex_invalid() {
        //Soo cannot reach seller
        let mut hashmap = HashMap::new();
        hashmap.insert("Soo", vec!["Bob", "Sally"]);
        hashmap.insert("Bob", vec!["Nao", "David"]);
        hashmap.insert("Sally", vec!["Joe", "Carrie"]);
        hashmap.insert("Nao", vec!["Jane", "Bly"]);
        hashmap.insert("Bly", vec!["July", "Bill"]);

        let test = bfs(&hashmap, "Soo");
        let expected = false;
        assert_eq!(test, expected);
    }

    #[test]
    fn with_seller_complex_valid() {
        //Soo can reach seller
        let mut hashmap = HashMap::new();
        hashmap.insert("Soo", vec!["Bob", "Sally"]);
        hashmap.insert("Bob", vec!["Nao", "David"]);
        hashmap.insert("Sally", vec!["Joe", "Carrie"]);
        hashmap.insert("Nao", vec!["Jane", "Bly"]);
        hashmap.insert("Bly", vec!["July", "Bill"]);
        hashmap.insert("Joe", vec!["Terry", "seller"]);

        let test = bfs(&hashmap, "Soo");
        let expected = true;
        assert_eq!(test, expected);
    }
}
