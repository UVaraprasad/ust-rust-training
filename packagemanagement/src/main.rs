use packagemanagement::greet; 
use  packagemanagement::shape::{greet_a_shape,greet_a_shape2};
use packagemanagement::shape::rect::rectangle::Rect;
use packagemanagement::shape::cuboid::cube::C1::C2::Cuboid;
//extern crate shapes;
use shapes::add;
//mod shapes="../shapes";
//include!("../shapes");

fn main() {
    println!("Hello, world!");
    greet();
    greet_a_shape();
    greet_a_shape2();
    let r1 = Rect::new(10.12, 12.34);
    let a1 = r1.area();
    let p1 = r1.perimeter();
    println!("Area of Rect:{} Perimeter of Rect:{}",a1,p1);
    println!("r1:{:?}",r1);
    let k = add(100, 200);
    println!("{}",k);
}


// create a library crate 
// cargo new shapes --lib
// lib.rs
//  shape.rs
//      shape/
//          rect/
//           rect.rs
//          square/
//              square.rs

// cargo build 
// creat a new project
// cargo new consumershape
// Find out how to call shape crate in this crate