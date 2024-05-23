fn main() {
 //let r1 = Rectangle{l:100.12,b:130.45,a:0.0,p:0.0};
let r1 = Rectangle::new(10.12, 13.45);
let mut r2= Rectangle::default();
r2.l=123.343;
r2.b=123.4354;
let a1=r1.area();
let p1 = r1.perimeter();
println!("Area of rect1:{a1}\nperimeter of rect1:{p1}");

let b1:Box<i32>= Box::new(100);

let u1 = Unit;
u1.greet();

let s1 = Square(25.45);

}

// normal struct
struct Rectangle{
    l:f32,
    b:f32,
    a:f32,
    p:f32,
}

impl Rectangle{
    fn new(l:f32,b:f32)->Self{
      Rectangle{l:l,b:b,a:0.0,p:0.0}  
    }

    fn default()->Self{
        Rectangle{l:0.0,b:0.0,a:0.0,p:0.0}  
      }

    fn area(&self)->f32{
        return self.l*self.b;
    }

    fn perimeter(&self)->f32{
        return 2 as f32 * (self.l+self.b);
    }
}

// new type  tuple struct
 struct Square(f32);

 // unit struct or empty struct
 struct Unit; 

 impl Unit{
    fn greet(&self){
        println!("Hi, I am calling Unit struct");
    }
 }

 // embedded struct
 struct Person{
    name:String,
    email:String,
    address:Address, // stru
 }

 struct Address{
    line1:String,
    country:String,
    pin_code:String,
  }


  struct A<'a>{ 
    n1:String,
    n2:&'a str, // not only abt &str
    n3:&'a i32,
  }