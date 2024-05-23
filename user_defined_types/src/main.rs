// #![no_std]
// I am writing non computer code. So dont use standard library
//#[derive(Debug)]
#[warn(unused_variables)]
fn main() {
    let x = 100;
    println!("Hello, world!");

    let x2: Option<i32> = Some(100);
    let east1: Direction = Direction::East;
    println!("{:#?}", east1); // unless the display traint is implemented it is not displayed using println! macro

    let message1 = Message::Quit;
    let message2 = Message::Move { x: 100, y: 200 };
    let message3 = Message::Write("Hello World".to_string());
    let message4 = Message::ChangeColor(56, 82, 23);
    print_message(&message1);
    // print_message(message2);
    // print_message(message3);
    // print_message(message4);
    message1.print();
    message2.print();
    message3.print();
    message4.print();

    let mut message5 = Message::Quit;
    message5.print(); 
    message5.move_to(100, 200);
    message5.print();
}

// enums



#[repr(C)]
#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        // refereing to the calling object
        match self {
            Message::Quit => {
                println!("quit")
            }
            Message::Move { x, y } => {
                println!("X:{x} Y:{y}");
            }
            Message::Write(text) => {
                println!("{text}");
            }
            Message::ChangeColor(r, g, b) => println!("r:{r} g:{g} b:{b}"),
        }
    }

    fn move_to(&mut self, x: i32, y: i32) {
        *self = Message::Move { x: x, y: y }; // dereference here
    }
}

fn print_message(msg: &Message) {
    match msg {
        Message::Quit => {
            println!("quit")
        }
        Message::Move { x, y } => {
            println!("X:{x} Y:{y}");
        }
        Message::Write(text) => {
            println!("{text}");
        }
        Message::ChangeColor(r, g, b) => println!("r:{r} g:{g} b:{b}"),
    }
}

fn get_Value(v: Option<i32>) {
    match v {
        Some(n) => {}
        None => {}
    }
}

// fn get_even_square(num:i32)->Result<i32,String>{

//     // if the number is even then give square
//     // if the number is not even then give an error message odd values are not squared here

// }
// core::result
// pub enum Result<T, E> {
//     Ok( /* … */ ),
//     Err( /* … */ ),
// }


#[derive(Debug, Clone)]
enum Direction {
    East,
    West,
    South,
    North,
}