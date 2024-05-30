use std::ops::Add;

impl Add for String {
    type Output = String;
    fn add(self, other: String) -> String {
        let mut result = self;
        result.push_str(&other);
        result
    }
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + s2;

    println!("{}", s3); // Output: Hello, world!
}

trait Math<T>{
    fn add(&self,t:T)->T;
}

struct<T> Number(T);

imp<T> Math<T> for Number<T>{

}