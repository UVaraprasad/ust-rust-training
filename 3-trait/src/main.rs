fn main(){
let mut r1 = Rect::<i32>{l:100,b:200};
let a1 = r1.area();
println!("{:?}",a1);
}

trait Area<T>{
    fn area( &self)->T;
}

trait Perimeter<T>{
    fn perimeter( &self)->T;
}

trait Shape<T>:Area<T>+Perimeter<T> where T:std::fmt::Debug{
    fn get_shape(&self){
        println!("Area::{:?}",self.area());
        println!("Perimeter:{:?}",self.perimeter());
    }
}

#[derive(Clone,Debug,Copy)]
struct Rect<T>{
    l:T,
    b:T,
}

impl<T> Area<T> for Rect<T> where T:std::ops::Mul<Output=T>+Clone{
    fn area(&self)->T {
       let l = self.l.clone();
       let b = self.b.clone();
       return l*b;
    }
}