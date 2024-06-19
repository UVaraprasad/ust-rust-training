use std::thread;
use std::time::Duration;
fn main() {
    let mut handlers = Vec::new();
    for i in 1..=10{
        let handle= thread::spawn(move||{
            for j in 1..=100{
                println!("{i} thread is running .Value :{j}");
                thread::sleep(Duration::from_millis(10));
            }
        });
        handlers.push(handle);
       
    }    
    for h in handlers{
        h.join().unwrap();
    }
    println!("All threads are done executing");

}
