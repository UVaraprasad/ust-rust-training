use std::time::Duration;
use std::{sync::mpsc, thread};
fn main() {
    let (tx1, rx1) = mpsc::channel();
    let tx2 = tx1.clone();
   // let (tx3,_) = mpsc::channel(); // tis is a new producer
   
    let handle1 = thread::spawn(move || {
        for i in 1..=10 {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 1000..=1010 {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
   
    
    let handle3 = thread::spawn(move || {
        for r in rx1 {
            println!("receiver-1:{r}");
        }
        println!("Everything is  received from receiver1");
    });
   
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
   
}
//Don't communicate by sharing memory; share memory by communicating
