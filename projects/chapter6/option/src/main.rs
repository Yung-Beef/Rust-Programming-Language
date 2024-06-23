// enum Option<T> { Option enum is already defined as part of the standard library and is automatically included
//     None, // Included in the preclude - can use Some and None directly without Option:: prefix
//     Some(T),
// }

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; // have to specify the type that the corresponding Some may eventually hold

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}
