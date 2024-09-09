use std::io;

// fn main() {
//     let x = 5;

//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x is {x}");
// }

    // let y: f32 = 3.0;
    // let sum = 5 + 3;
    // let t = true;
    // let f: bool = false;
    // let c = 'z' // uses single quotes for chars
    // let tup: (i32, f64, u8) = (500, 6.4, 1); // specify the data types when creating the tuple
    // let (x, y, z) = tup;
    // println!("The value of y is {y}");
    // let five_hundred = tup.0; // indexing into the tuple
    // let six_point_four = tup.1;
    // let one = tup.2;
    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // array of size 5 of all i32s
    // let a = [3; 5]; // same as "let a = [3, 3, 3, 3, 3];"
    // let second = a[1];

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}