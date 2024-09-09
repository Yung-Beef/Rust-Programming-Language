// fn main() {
//     println!("Hello, world!");

//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(x: i32, unit_label: char) {
//     println!("The measurement is: {x}{unit_label}");
// }


// fn five() -> i32 { // declaring the type of return value
//     5 // no semicolon because it is an expression, and is at the end of the function so it will be returned
// }

// fn main() {
//     let x = five();

//     println!("The value of x is : {x}");
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}