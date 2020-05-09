fn main() {
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

    let mut count = 0;
    let coin = Coin::Quater(UsState::Alabama);

    match &coin {
        Coin::Quater(state) => println!("Hello, world!"),
        _ => count += 1,
    };

    if let Coin::Quater(state) = &coin {
        println!("Hello, world!");
    } else {
        count += 1;
    }
}

