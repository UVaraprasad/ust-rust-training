pub fn greet_a_shape(){
    println!("hello this is my greet a shape fn");
}

pub fn greet_a_shape2(){
    println!("hello this is my greet a shape fn");
}

pub mod rect;
pub mod square;
pub mod cuboid;

// packagemanagement
//     lib.rs
//        shape.rs
//          shape/rect.rs
//               /square.rs