use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites 10
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Blue")).or_insert(50); // if Blue doesn't exist yet, add it with 50 score, otherwise do nothing
    scores.entry(String::from("Yellow")).or_insert(50); // entry method returns an Entry enum, is a value that may exist or not

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns a mutable reference to the value of that key
        *count += 1; // so count needs to be dereferenced to update the actual value in the hashmap
    }
    
//  let team_name = String::from("Blue");
//  let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field name and value are invalid after this line b/c Strings don't have Copy trait
    // insert references into the hashmap to avoid moving the values, but the refs must be valid as long as the hashmap is

}
