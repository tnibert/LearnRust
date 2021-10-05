#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    let x = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // thhis will never run
        Coin::Penny => {
            println!("jackpot");
            100
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    };
    x
}

fn main() {
    // let amount = value_in_cents(Coin::Penny);
    let amount = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("amount: {}", amount);
}