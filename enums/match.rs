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
        // this will never run
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
    // can define function inside function
    // match for Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // the _ placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // let amount = value_in_cents(Coin::Penny);
    let amount = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("amount: {}", amount);
}