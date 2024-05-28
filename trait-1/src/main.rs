fn main() {
    
    let num1:i32 = 32;

    let sq = num1.square(); // This is a method that has been called on i32 by implementing Math trait

    println!("Square of i32: {}",sq);

    let num2 = Number(33);

    let sq = num2.square();

    println!("Square of Number: {}",sq);


    let val1 = Value{num:34};

    let sq = val1.square();

    println!("Square of Value: {}",sq);

}

trait Math {
    fn square(&self) -> i32;
}

// 1. Normal struct

#[derive(Debug)]
struct Value {
    num: i32,
}

impl Math for Value {
    fn square(&self) -> i32 {
        self.num * self.num
    }
}

#[derive(Debug)]
struct Number(i32);

impl Math for Number {
    fn square(&self) -> i32 {
        return self.0 * self.0;
    }
}

// can also implement trait on primitive types

impl Math for i32 {
    fn square(&self) -> i32 {
        return self * self;
    }
}

// 