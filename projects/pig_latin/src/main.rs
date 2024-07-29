use std::io;
use regex::Regex;

//fn main() {
    // create string
    // let mut input = String::new();
    // let mut output: Vec<String> = vec![];
    
    // loop {
    //     // get input
    //     println!("Please enter a string to convert to pig latin");
    //     io::stdin().read_line(&mut input).expect("Unable to read input.");

    //     'word: for word in input.split_whitespace() {
    //         let mut v = vec![];
    //         let mut deq = VecDeque::new();
    //         for letter in word.chars() {
    //             if !letter.is_alphabetic() {
    //                 output.push(letter.to_string());
    //                 //continue 'word
    //             }
    //             v.push(letter);
    //             deq.push_back(letter);
    //         }
    //         if matches!(v[0], 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' ) {
    //             v.push('h');
    //             v.push('a');
    //             v.push('y');
                
    //             // add to output
    //             output.push(v.into_iter().collect());
                

    //         } else {
    //             let f = deq.pop_front().unwrap();
    //             deq.push_back(f);
    //             deq.push_back('a');
    //             deq.push_back('y');
                
    //             // add to output
    //             output.push(deq.iter().collect());
    //         }
    //     }
    //     break
    // }

    // for word in output {
    //     println!("{}", word);
    // }
    
    // split on whitespace into interator
    

    // iterate and make changes and print as you go
    
//}

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
    word.to_string().replace(/(\w)(\w+)/g, '$2$1ay')
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