fn add_five<F>(func: F)
    where F: Fn(i32) {
        func(5)
    }

fn main() {
    let num1 = 6;
    add_five(|x| println!("{}", x));
}
