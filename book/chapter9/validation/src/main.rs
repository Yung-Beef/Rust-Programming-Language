pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    loop {
        // --snip--
        let guess: i32 = match guess.trim()parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number is between 1 and 100");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
        }
    }
}
