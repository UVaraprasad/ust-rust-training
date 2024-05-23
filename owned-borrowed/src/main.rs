fn main() {


    let mut x = 10;
    let _y = &x;

    x = 100;
    //println!("{y}");
    println!("{x}");
}

// 1-
// Ownership
// A variable owns a value
// There is one owner for a value at a time
// When we assign this value , we move the value and change the owner

// Ownership rules
// Each value in Rust has an owner, which is usually a variable.
// There is only one owner of the value at a time.
// When the variable (owner) goes out of scope,
//   the value goes away and is no longer available (or accessible) 
//   unless the value changes owner beforehand.

// Points to explain or discuss
// Lifetime of a value
// Lifetime of a reference
// Make a reference to a value
// Rust Owned And Borrowed Types
// Non Lexical Lifetimes