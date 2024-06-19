use std::thread;
use std::time::Duration;

static mut BALANCE:i32=100;
fn main() {

    //let mut counter = 0;

    let handle1 = thread::spawn(move ||{
        for i in 1..=10{
            unsafe{
                BALANCE=BALANCE+1;
                println!("Thread1:{}",BALANCE);
                thread::sleep(Duration::from_millis(100));
            }
        }
    });

    let handle2 = thread::spawn(||{
        for i in 1..=10{
            unsafe{
                BALANCE = BALANCE-1;
                    println!("Thread2:{}",BALANCE);
                    thread::sleep(Duration::from_millis(100));
            }
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    unsafe{
        println!("Final:{}",BALANCE);
     }
   
}


// what is the reason for data race ?
// why do we need to look for alternate ways to deal with data race?
// 