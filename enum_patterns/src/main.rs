enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function(&self) {
        println!("{:?}", self);
    }
}

struct IpAdd {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let four = IpAddressKind::V4(1,2,3,4);
    // let six = IpAddressKind::V6(String::from("123.7.8.9"));

    // let localhost = IpAdd {
    //     kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let mess = Message::Move { x: 2, y: 2 };

    mess.some_function();

    let mess2 = Message::Quit;
    mess2.some_function();

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(45);
    let some_string = Some("a string");
    let absent: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(10);

    let sum = x + y.unwrap_or(0); // y can be undefined as it is optional
    println!("Sum = {}", sum);

    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let pls_one = plus_one(Some(3));

    println!("{:?}", pls_one);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 1,
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter(state) => {
            println!("State {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // _ => None // we can use this too for none
        Some(i) => Some(i + 1),
    }
}
