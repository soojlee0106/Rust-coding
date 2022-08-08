//Sorting the sentence
pub fn sort_sentence(s: String) -> String {
    let mut firstvector: Vec<&str> = s.split(" ").collect::<Vec<&str>>();

    firstvector.sort_by(|a, b| a.chars().last().unwrap().cmp(&(b.chars().last().unwrap())));

    let newindex = firstvector
        .iter()
        .map(|x| String::from(x.split_at(x.len() - 1).0));

    let finalvector: Vec<String> = newindex.collect();

    finalvector.join(" ")
}

//Count Items Matching a Rule
pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let mut matches = 0;

    let x: &str = &rule_key;

    let i = match x {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return 0,
    };

    for j in items {
        if j[i] == rule_value {
            matches += 1;
        }
    }

    matches
}
