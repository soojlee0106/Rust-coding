//Decode the Message
use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut hashmap = HashMap::new();
        let mut alphabet = 'a'..='z';

        for k in key.chars().filter(|&c| c != ' ') {
            if let Entry::Vacant(v) = hashmap.entry(k) {
                v.insert(alphabet.next().unwrap());
            }
        }

        message
            .chars()
            .map(|c| *hashmap.get(&c).unwrap_or(&c))
            .collect()
    }
}

//Convert Binary Number
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut integer = 0;
        let mut currentnode = head;

        while let Some(node) = currentnode {
            integer *= 2;
            integer += node.val;
            currentnode = node.next;
        }

        integer
    }
}
