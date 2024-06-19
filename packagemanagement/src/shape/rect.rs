use crate::shape::greet_a_shape;
use crate::greet;
pub mod rectangle {
    #[derive(Debug)]
    pub struct Rect {
        pub l: f32,
        pub b: f32,
    }

impl Rect{
    pub fn new(l:f32,b:f32)->Self{
        Rect { l: l, b: b }
    }
    pub fn area(&self)->f32{
        return self.l*self.b;
    }
    pub fn perimeter(&self)->f32{
        return 2.0*(self.l+self.b);
    }
    pub fn greet_func(){
        super::greet_a_shape();
        super::greet();
    }
}
}
