use std::io;
use medianmode::functions;

fn main() {
    // get input (like "55 32 5 10 10 30")
    let mut input = String::new();
    let mut v: Vec<i32> = Vec::new();
    'outer: loop {
        // Get user input and store it in input
        println!("Please enter a list of integers separated by spaces");
        io::stdin().read_line(&mut input).expect("Unable to read input.");

        // ensure the user inputs something
        if input.trim().is_empty() {
            continue 'outer;
        }

        // add each number to v, ask for input again if they enter anything that isn't a number or space
        for number in input.split_whitespace() {
            match number.parse::<i32>() {
                Ok(num) => v.push(num),
                Err(_) => continue 'outer,
            }
        }
        break
    }
    
  
    let median = functions::median(&v);
    println!("Median: {}", median);

    let mode = functions::mode(&v);
    println!("Mode: {}", mode);
}
