fn main() {
    let r1 = Rect::new(12.34, 13.45);
    shape_of(&r1);
    println!("{:?}",r1);
    let s1 = Square::new(25.0);
    shape_of(&s1);    
}

fn shape_of(o: &impl Shape){

 o.display();

 let a = o.area();
 println!("Area:{a}");

 let p = o.perimeter();
 println!("Perimeter:{p}");

}

trait Shape: Area+Perimeter{
   fn display(&self);
}

trait Area{
    fn area(&self)->f32;
}

trait Perimeter{
    fn perimeter(&self)->f32;
}



// self, &self, &mut self and Self

#[derive(Debug)]
struct Rect{
    l:f32,
    b:f32,
}

impl Area for Rect{
    fn area(&self)->f32 {
        return self.l * self.b;
    }
}

impl Perimeter for Rect{
    fn perimeter(&self)->f32 {
     2.0*(self.l+self.b) 
    }
}

impl Shape for Rect{
    fn display(&self) {
        println!("Rect length:{} bredth:{}",self.l,self.b);
    }
}


impl Rect{
    fn new(l:f32,b:f32)->Self{
        Rect { l: l, b: b }
    }
    fn default()->Self{
        Rect { l: 0.0, b: 0.0 }
    }
}

#[derive(Debug)]
struct Square(f32);

impl Area for Square{
    fn area(&self)->f32 {
        return self.0 * self.0;
    }
}

impl Perimeter for Square{
    fn perimeter(&self)->f32 {
        self.0 * 4 as f32
    }
}

impl Shape for Square{
    fn display(&self) {
        println!("Square side:{}",self.0);
    } 
}

impl Square{
    fn new(s:f32)->Self{
        Square(s)
    }
}