fn main() {
    let x = 100;
    let y = &x;
    println!("{} {}", x, y);

    let mut x2 = 100;   // x2 is the owner of 100
    
    let mut y2 = &mut x2; // y2 is a mutable reference and y2 has borrowed x2 
                                    // temporarly the owner of 100 is y2 
    
    let z2  = &mut *y2;   // z2 id the owner of the data since there is a borrow from y2

   // *y2 = *y2+1;
    *z2 = 200;
   //x2 = x2+1;

    println!("{}",z2);
}
// 