fn main() {
    println!("Hello, world!");

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
// 枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("it is Penny");
            1
        }
        Coin::Nickel => 10,
        Coin::Dime => 5,
        Coin::Quarter(state) => {
            println!("the state quarter is :{:?}", state);
            15
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some((i)) => Some(i + 1),
    }
}
