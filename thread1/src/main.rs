use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

   let handler=thread::spawn(||{
        for i in 1..10{
            println!("From a spawn thread:{}",i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..5{
        println!("From a main thread:{}",i);
        thread::sleep(Duration::from_millis(10));
    }
    handler.join().unwrap();
}
