#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // following comma is optional if using {}
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // other => move_player(other), // uses the values besides 3 or 7
    _ => reroll(), // doesn't bind the dice value
    // _ => () // nothing happens unless you roll 3 or 7, () is an empty tuple, the "unit value"
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}
fn reroll() {}