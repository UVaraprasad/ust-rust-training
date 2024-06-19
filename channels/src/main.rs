use std::{sync::mpsc, thread, time::Duration};

fn main() {
    
   let (tx1,rx1)= mpsc::channel();

   let tx2 = tx1.clone();
   thread::spawn(move||{
        let s1 = "Hello";

        for i in 1..=10{
            tx1.send(i);
            thread::sleep(Duration::from_secs(1));
        }
      
   });

   thread::spawn(move||{
    let s1 = "Hello";
let mut i = 1;
loop{
    //for i in 100..=110{
        tx2.send(i);
        thread::sleep(Duration::from_secs(2));
    }
  
});

   //let v = rx.recv().unwrap();
   

   for v in rx1{
    println!("v:{}",v);
   }

}
