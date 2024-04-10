#![allow(warnings)]

pub fn run_enum() {
    // enum with different data types
    let m1 = Message::Write(String::from("hello"));
    m1.call();
    let m2 = Message::Move { x: 1, y: 2 };
    m2.call();
    let m3 = Message::ChangeColor(1, 2, 3);
    m3.call();
    let m4 = Message::Quit;
    m4.call();

    // option enum matching
    let number = Some(5);
    assert!(plus_one(number) == Some(6));
    let no_number: Option<i32> = None;
    assert!(plus_one(no_number) == None);

    // if let
    assert!(plus_one_v2(number) == Some(6));
    assert!(plus_one_v2(no_number) == None);

    // complex matching
    let quarter = Coin::Quarter(UsState::Alaska);
    assert!(value_in_cents(quarter) == 25);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Unknown1,
    Unknown2,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        other => {
            println!("Unknown coin: {:?}", other);
            0
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn plus_one_v2(x: Option<i32>) -> Option<i32> {
    if let Some(i) = x {
        Some(i + 1)
    } else {
        None
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}
