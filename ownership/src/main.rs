
static mut GV:i32= 100;

fn main() {
    let k: i32 = 100;

    let mut l: i32 = k; // deep copy

    l = l + 1;

    // k is the owner of 100
    // l is alao the owner of another 100

    let mut s1 = String::from("Hello World"); // s1 is the owner of "Hello World"

    let mut s2 = s1; // The ownership of "Hello World" is transferred to s2;

    //s1 = s2;

    //println!("S1:{}",s1)

    println!("S2: {}", s2);
    s2 = greet1(&s2);

    println!("S2: {}", s2);// Hello World How are you doing

    s2=greet2(s2); // Who is the owner of this data --> "Hello World How are you doing"
    println!("S2: {}", s2);

    greet3(&mut s2);

    println!("S2: {}", s2);

}

fn greet1(input: &str) -> String {
    let mut s = input.to_string(); // create another variable
    s.push_str(" How are you doing"); // mutated
    return s;
}

fn greet2(mut input: String) ->String{
    input.push_str("The whold world should say Hello world");
    return input;
}

fn greet3(input: &mut String){
    input.push_str("The whold world should say Hello world");
}


fn get_len(input: &String){
  println!("Length:{}",input.len()); // not mutating the string hence no need of mutable reference of input
}

// Ownership rules
// Each value in rust has a variable, that variable is called its Owner.
// There can only be one owner at a time.
// When the owner goes out of scope , the value is dropped. // To drop rust internally call drop trait

// Ownership transfer
// Onwership is tranffered using moves
// When the value is assigned to another variable or passed to a function, the ownership is transffered.
// This prevents data races. 

// Borrowing
// Instead of transffering the ownership, you can borrow a reference to a value
// Borrowing allows you to temporarily use a value without taking the ownership
// references are immutable by default, to avoid accidental modifications.
