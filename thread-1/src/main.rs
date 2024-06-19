use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let handle1 = thread::spawn(|| {
        for i in 1..=10 {
            println!("new thread1 {i}");
            thread::sleep(Duration::from_millis(5));
        }
    });

    let handle2=thread::spawn(|| {
        for i in 1..=10 {
            println!("new thread2 {i}");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..=5 {
        println!("Main thread {i}");
        thread::sleep(Duration::from_millis(5));
    }
    handle1.join().unwrap();
    handle2.join().unwrap();

}
