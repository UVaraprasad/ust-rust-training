fn main() {

   // let mut num1= Number::new(100);
    let mut num1 = Number::default();
    let total = num1.add(100).add(200).sub(100).add(300).add(400).sub(300).get();
    println!("Total:{}",total);
}

trait Math{
    fn get(&self)->f32;
    fn add(&mut self,num:f32)-> &mut impl Math;
    fn sub(&mut self,num:f32)-> &mut impl Math;
}

struct Number{
    num:i32
}

impl Number{
    fn new(num:i32)->Self{
        Number { num: num }
    }
    fn default()->Self{
        Number { num: 0 }
    }
}

impl Math for Number{
    fn get(&self)->i32{
        return self.num;
    }
    fn add(&mut self,num:i32)->&mut impl Math{
        self.num+=num;
        return self;
    }
    fn sub(&mut self,num:i32)->&mut impl Math{
        self.num-=num;
        return self;
    }
}