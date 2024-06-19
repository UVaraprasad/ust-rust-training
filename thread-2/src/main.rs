use std::{thread, time::Duration};

fn main() {

let mut handles = Vec::new();
let mut s1: String = String::from("hello World");
    for i in 1..=100{
       let handle= thread::spawn(move ||{
            for j in 1..=10{
                thread::sleep(Duration::from_millis(10));
                println!("thread:{i}->>{j}");
            }
        });

        s1.push_str("hey");
        println!("{s1}");
        handles.push(handle);
    }

    for v in handles{
        v.join().unwrap();
    }
// this is blocked here until all threads finish their execution
    println!("{s1}");
    println!("I am done from main");
}
