fn main() {
    let one_penny = Coin::Quarter(UsState::Alabama);
    println!("{}", one_penny.value_in_cents());

    println!("{:?}", plus_one(None));
    println!("{:?}", plus_one(Some(5)));
}

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

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // {:?} formats the "next" value passed to a formatting macro, and supports anything that implements Debug.
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
