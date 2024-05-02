fn main() {
    let mut name1: String = String::from("UST Global");
    let mut name2: String = String::new();
    let mut name3: String = "".to_string();

    println!("{}{}{}", name1, name2, name3);

    if name1 != "" {
        println!("name1 has nothing");
    }
    if name2 != "" {
        println!("name2 has nothing");
    }
    if name3 != "" {
        println!("name3 has nothing");
    }

    // there is no Null or nil or null in Rust.
    let mut comp_name1: Option<String> = Some(String::from("UST Glocal"));
    let mut comp_name2: Option<String> = None;
    let mut number1: Option<i32> = None;
    let mut number2: Option<Box<i32>> = None;
    let mut number3:Option<i32>=Some(100);
    let mut number4: Option<Box<i32>> = Some(Box::new(100));


    match comp_name1 {
        Some(n) => println!("{}", n),
        None => println!("None"),
    }
    increment1(number1);
    increment1(number3);
  //  increment2(number2);
    increment2(&mut number4);
    println!("{:?}",number4);


}

fn increment1(num: Option<i32>) {
    match num {
        Some(mut n) => {
            n += 1;
            println!("{}", n)
        }
        None => {
            println!("None")
        }
    }
}

fn increment2(num: &mut Option<Box<i32>>) {
    match num {
        Some(ref mut n) => {
            **n += 1;
            println!("{}", **n);
        }
        None => {
            println!("None");
        }
    }
}