use std::time::Duration;
use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let (tx2,rx2)= mpsc::channel();
    let handle2 = thread::spawn(move || {
        for r in rx {
            println!("receiver-1:{r}");
            tx2.send(r).unwrap();
        }
        println!("Everything is  received from receiver1");
    });
    let handle3 = thread::spawn(move || {
        for r in rx2 {
            println!("receiver-2:{r}");
        }
        println!("Everything is received from receiver2");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
//Don't communicate by sharing memory; share memory by communicating
