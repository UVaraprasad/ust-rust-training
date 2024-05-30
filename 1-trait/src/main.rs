fn main() {

let mut num1 = Number::new(100);

let result = num1.add(100).add(200).add(300).get();

println!("{}",result);
}



trait Math{
    fn add(&mut self,num:i32) -> &mut dyn Math;
    fn get(&self)->i32;
}

#[derive(Debug)]
struct Number(i32);

impl Number{
    fn new(num:i32)->Self{
        Self(num)
    }
    fn default()->Self{
        Self(0)
    }
}

impl Math for Number{
    fn add(&mut self,num:i32) -> &mut dyn Math {
        self.0 = self.0+num;
        return self;
    }
    fn get(&self)->i32 {
        return self.0;
    }
}