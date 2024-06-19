pub mod C1{
    pub mod C2{
        pub struct Cuboid{
            pub l:f32,
            pub b:f32,
            pub h:f32
        }

        impl Cuboid{
            pub fn new(l:f32,b:f32,h:f32)->Self{
                Cuboid { l: l, b: b,h:h}
            }
            pub fn area(&self)->f32{
                return self.l*self.b *self.h;
            }
            pub fn perimeter(&self)->f32{
                return 2.0*(self.l+self.b+self.h);
            }
        }
    }
}