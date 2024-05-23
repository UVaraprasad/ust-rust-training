fn main() {
    let r1 = Rect::new(12.34, 13.45);
    area_of(&r1);
    println!("{:?}",r1);
    let s1 = Square::new(25.0);
    area_of(&s1);

    

    // let k = &mut 100; // create memory to store i32 and store 100 in it. No name for that only a reference
    // *k = 101;

    // let num1 = 100;
    // let ptr1 = &num1;

    // let ptr2 = &500;

    
}

fn area_of(o: &impl Shape){
 let a = o.area();
 println!("Area:{a}");
}

trait Shape{
    fn area(&self)->f32;
    fn perimeter(&self)->f32;
}
// self, &self, &mut self and Self

#[derive(Debug)]
struct Rect{
    l:f32,
    b:f32,
}

impl Shape for Rect{
    fn area(&self)->f32 {
        return self.l * self.b;
    }

    fn perimeter(&self)->f32 {
         2.0 * (self.l+self.b)
    }
}

impl Rect{
    fn display(&self){
        println!("Rect Length:{}, Bredth:{}",self.l,self.b);
    }

    fn new(l:f32,b:f32)->Self{
        Rect { l: l, b: b }
    }

    fn default()->Self{
        Rect { l: 0.0, b: 0.0 }
    }
}

#[derive(Debug)]
struct Square(f32);

impl Shape for Square{
    fn area(&self)->f32 {
        return self.0 * self.0;
    }
    fn perimeter(&self)->f32 {
        self.0 * 4 as f32
    }
}

impl Square{
    fn new(s:f32)->Self{
        Square(s)
    }
}