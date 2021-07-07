fn main() {
    let coin = Coin::Penny;
    println!("Penny is {}", value_in_cents(&coin));
    let coin = Coin::Nickel;
    println!("Nickel is {}", value_in_cents(&coin));
    let coin = Coin::Dime;
    println!("Dime is {}", value_in_cents(&coin));
    let coin = Coin::Quarter(UsState::Alabama);
    println!("Quarter Alabama is {}", value_in_cents(&coin));
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Quarter Alaska is {}", value_in_cents(&coin));

    println!("==================================");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {:?}", six);
    println!("none is {:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
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
