// fn main() {
//     // loop {
//     //     println!("again!");
//     // }

//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining  == 9 {
//                 break;
//             }
//             if count == 2 { 
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }

//     println!("End count = {count}")
// }


// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFF!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}