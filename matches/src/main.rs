#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn main() {
    let coin = Coin::Quater(UsState::Alaska);

    value_in_cents(coin);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}
