use std::{sync::Mutex,thread};
use std::sync::Arc;



fn main() {

    let  mu1 = Arc::new( Mutex::new(1));
   
   let thread1_mu1 = Arc::clone(&mu1);
   let handle1= thread::spawn(move||{
    let mut k = thread1_mu1.lock().unwrap();
    *k+=1;
    
    });
    let thread1_mu2 = Arc::clone(&mu1);
  
    let handle2=thread::spawn(move||{
        let mut k = thread1_mu2.lock().unwrap();
        *k+=1;
    });
    handle1.join();
    handle2.join();

    println!("{:?}",*mu1.lock().unwrap());
    println!("exiting main")

}
