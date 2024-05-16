fn main() {
    let mut x = 100;

    let y = &x; // - x is borrowed here

    println!("{}",x); // borrow later used here
    println!("{}",y);
}

// :