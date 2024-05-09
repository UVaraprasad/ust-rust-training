fn main() {
    println!("Hello, world!");

    let age: u8 = 85; // 1byte

    let num1: u16 = age as u16; // two bytes

    // there are two traits those are used for type casting
    // into() from() these are used for general type conversions

    let ok1: bool = false;

    let num2: u8 = ok1 as u8;

    println!("{}", num2);

    let num3: u8 = 100;

    let ok2: bool = num3 != 0;

    let a: char = '也'; // all normal english chars follow ascii.
    let num4: u16 = a as u16;

    println!("{}", num4);

    let num5: u16 = 20063;

    let num6: u8 = 97;
    let char2: char = num6 as char;

    let char3 = char::from_u32(num5 as u32);

    //Some(v),None

    match char3 {
        Some(ch) => println!("{}", ch),
        None => {}
    }

    let num7: u64 = 1231231123;
    let num8: u8 = num7 as u8;

    println!("{}", num8);

    //let b:u8=0b10010010;
    let b: u8 = 0b10010011;

    println!("{}", b);

    let str_num = "a3.145";
    let r: Result<f64, std::num::ParseFloatError> = str_num.parse::<f64>();

    // write a function to convert from str to f64
    // 1. It can convert bcz it is a number in the form of string
    // 2. It cannot convert bcz it is not a number
    // 3. It cannot convert if None or no value is given, the string is ""
    // 3.1 If the str is empty you can convert it as 0

    // Error trait 
    // Error : Debug + Display 
    // Error methods
    // Debug methods
    // Display methods


  


    match r {
        Ok(v) => {
            println!("{}", v)
        }
        Err(e) => {
            println!("there seems to be an error {}", e)
        }
        // Err(_) => {
        //     println!("there seems to be an error");
        // }
    }

    let t = (36,"Hello World",false);

    println!("{:?}",t);
}

// implicit & Explicit

// implement from and into trait for this example

// bool, u8 to u64 and i8 to i64 , f32 and f64 , char , string with "100"
// sum:f64 =

// _ is blank identifer