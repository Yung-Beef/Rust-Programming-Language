use std::io; //for stdin()

fn main() {
    loop {
        // get input
        let mut n = String::new();

        println!("Please enter the sequential number of which Fibonacci number you would like to see.");
        io::stdin().read_line(&mut n).expect("Failed to read line");

        // trim parse to u128
        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let f = fibonacci(n);
        println!("Value {} in the Fibonacci sequence is {}.", n, f);
        break
    };
}

fn fibonacci(n: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = 0;
    if n == 0 {
        return a
    } else if n == 1 {
        return b
    } else {
        for _ in 1..n {
            c = a + b;
            a = b;
            b = c;
        }
        return c
    }
}
