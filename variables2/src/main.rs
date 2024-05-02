fn main() {

    let mut message: &str= "Hello World";

    println!("Length:{}",message.len());

    message = "Hello Universe"; // This is to ease the life of a developer

    println!("Message:{}",message);

   let message1 = message; // not a shallow copy; It is a deep copy

   println!("new string:{}",message1);

   message="Hello All of you";

   println!("new string:{}",message1);

//    let mut k:i32 = 100;
//     k=1231;
//     k=4322;
//     k=44325;

//     let mut  ok = false;
//     ok=true;

    let s2 = String::from("世界 您好");
    let s3 = message.to_string(); // stack allocated string is asssigned to heap allocated string
    
    let mut s4= String::new();
    s4.push_str("世界 您好");

    println!("s4:{}",s4);

    s4 = s4 + ";How are you"; // internally + operator has been implemented to concat strings

    println!("s4:{}",s4);
    
    let l=get_string_length(&s4);
    println!("Length of the string:{}",l);
    let l1=get_string_length(message);
    println!("Length of the string:{}",l1);

}


fn get_string_length(input: &str)->u32{
    return input.len() as u32;    // converting a usize to u32
}


// string in stack or in heap
// &str --> string slice. This is allocated in stack memory
// String --> String . This is allocated in heap memory


// str structure
// Pointer : The address of the string . 8 bytes
// Length: number of bytes . 8 bytes


// create s1 as &str 
//  s1 = s1 + ";How are you";