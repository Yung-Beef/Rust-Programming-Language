use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Please enter a string to convert to pig latin");
    io::stdin().read_line(&mut input).expect("Unable to read input.");

    println!("{}", pig_latin(input));
}

fn pig_latin(input: String) -> String {
    let mut output: Vec<String> = vec![];
    
    for word in input.split_whitespace() {
        let firstletter = first_letter(word).unwrap();
        if firstletter.is_alphabetic() {
            if matches!(firstletter, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' ) {
                output.push(start_vow(&word));
            } else {
                output.push(start_con(&word));
            }
        } else {
            output.push(word.to_string());
        }
    }
    // converts outvec to a string and returns it
    output.join(" ")
}

fn start_con(word: &str) -> String {
    let f = &word.chars().nth(0).unwrap().to_string();
    let end = &word[1..];
    let new = end.to_owned() + f + "ay";
    new
}

fn start_vow(word: &str) -> String {
    format!("{}{}", word, "hay")
}

fn first_letter(word: &str) -> Option<char> {
    for letter in word.chars() {
        return Some(letter);
    }
    None
}