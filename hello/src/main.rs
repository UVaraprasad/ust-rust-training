static mut GV:i64=123123131;
fn main() {
    println!("Hello, world!"); 
    // println! os a macro
    let mut num1 = 100;
    let num2: i64=12312312;

    {
        let k:i32=100;
    }
    // drop k from memory

    let float1: f32 = 1231.34;

    let float2: f64 =12313.1231231;

    let ok:bool = true;

    println!("{4},{0},{1},{2},{3},{5},{6}",num1,num2,float1,float2,ok,GV,pi);

    const pi:f32=3.14;

    num1 = 200;
    unsafe{
    GV = 12312;
    }
}
// variables are created in stack 
// variables are created in heap
// values are not inferred to a variable

// integers
// i8,i16,i32,i64,i128
// u8,u16,u32,u64,u128
// f32,f64

// boolean type
// bool 

// character type
// char

//Compund type:
// Tuple

// Strings
// &str


