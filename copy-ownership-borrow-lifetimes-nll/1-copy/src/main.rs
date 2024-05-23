fn main(){

    let x: i32 =100; // x is the owner of 100
    let y: i32 = x;  // y is the owner of 100

    let s1: String = String::from("Hello World!"); // s1 is the owner of 
    let s2: String = s1.clone(); // s2 is the owner of

    let b1:Box<i32>= Box::new(100); // b1 is the owner of
    let b2: Box<i32> = b1.clone(); // b2 is the owner of

}
