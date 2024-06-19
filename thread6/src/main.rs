use std::{sync::Mutex, thread,sync::Arc,rc::Rc};

fn main() {

    let  counter1 = Arc::new(Mutex::new(0));
    //let mut counter4 = Rc::new(Mutex::new(0));
    let counter2 = Arc::clone(&counter1);
    let counter3 = Arc::clone(&counter1);
    let handle1 = thread::spawn(move||{
        for i in 1..10{
           let mut k= counter2.lock().unwrap();
            *k = *k+10;
        }
    });

    let handle2 = thread::spawn(move ||{
        for i in 1..10{
            let mut k= counter3.lock().unwrap();
             *k = *k+10;
         }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{:?}",*counter1.lock().unwrap());

}

//rc --> does not work bcz it is not a thread safe
// arc works..
