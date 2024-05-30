use std::ops::Add;

// impl Add for String {
//     type Output = String;

//     fn add(self, other: String) -> String {
//         let mut result = self;
//         result.push_str(&other);
//         result
//     }
// }
#[derive(Debug)]
struct MyString1(String);

impl Add for MyString1{
    type Output = MyString1;
    fn add(self, rhs: Self) -> Self::Output {
        return MyString1(self.0+&rhs.0);
    }
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    println!("{}", s3); // Output: Hello, world!

    let s1 = MyString1("Hello".to_string());
    let s2 = MyString1(" World!".to_string());

    let mut s3 =s1+s2;

    println!("{:?}",s3);

    let num1 = Number::<i32>(100);
    let sum = num1.add(100);
    println!("{}",sum);

    let p1 = Number::<Point>(Point { x: 101, y: 200 });
    
    p1.add(Point{x:100,y:200});
    println!("{:?}",p1);
    // traits bounds are not satisfied
}

#[derive(Debug,Copy,Clone)]
struct Point{
    x:i32,
    y:i32,
}

impl std::ops::Add for Point{

    type Output = Point;

    fn add(mut self, rhs: Self) -> Self::Output {

        self.x = self.x + rhs.x;
        self.y = self.x+rhs.y;

        return self;
    }

}

trait Math<T>{
    fn add(self,t:T)->T;
}
#[derive(Debug,Copy,Clone)]
struct Number<T>(T);

impl<T: > Math<T> for Number<T> where T:std::ops::Add<Output = T>{
  fn add(mut self,t:T)->T {
        return self.0+t;
  }
}