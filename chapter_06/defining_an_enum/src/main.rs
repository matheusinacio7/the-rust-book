fn main() {
    println!("Hello, world!");
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: -6 };


    println!("{:?}", msg2);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(u8, u8, u8),
}
