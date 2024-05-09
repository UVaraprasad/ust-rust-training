fn main() {
    println!("Hello, world!");

    let mut counter = 0;

    loop{
        println!("Counter:{}",counter);
        counter+=1;

        if counter==10{
            break;
        }
    }

    let mut counter = 0;

    while counter<10{
        println!("{}",counter);
        counter+=1;
    }

    let message: &str = "Hello 您好!";
   //let message: &str = "Hello World!";
    let mut  rmsg = String::new();

    for c in message.chars(){
       // println!("{}",c);
        rmsg = c.to_string()+&rmsg; // to ease our life
       // rmsg.push(c);
    }
    println!("{}",rmsg);

    for i in message.bytes(){
        print!("{}",i as char);
    }

    println!();   

}

// for{

// }