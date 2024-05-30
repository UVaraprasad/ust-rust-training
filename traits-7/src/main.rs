fn main() {
    let r1 = Rect::new(10.5, 12.6);
    r1.get_area_perimeter();
}

trait Area{
    fn area(&self)->f32{
        return 1.0;
    }
}

trait Perimeter{
    fn perimeter(&self)->f32{
        return 1.0;
    }
}

trait Shape:Area+Perimeter{ // kind of multilevel in heritence
    fn get_area_perimeter(&self){
       let a = self.area();
       let p = self.perimeter();
       println!("Area:{} Perimeter:{}",a,p)
    }
}

#[derive(Debug,Copy,Clone)]
struct Rect{
    l:f32,
    b:f32
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        Self { l: b, b: b }
    }
}


impl Area for Rect{
    fn area(&self)->f32 {
        return self.b *self.l
    }
}
impl Perimeter for Rect{
    fn perimeter(&self)->f32 {
        return 2.0 *(self.l+self.b);
    }
}

impl Shape for Rect{}

//pub trait Add<Rhs = Self>
impl std::ops::Add for Rect{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            l: self.l + rhs.l,
            b: self.b + rhs.b,
        }
}
}