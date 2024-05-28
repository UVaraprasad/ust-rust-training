fn main() {

    // let mut num1= Number::new(100);
     let mut num1 = Number::default();
   //  let total: <dyn Math as Math>::T = num1.add(100);//.add(200).sub(100).add(300).add(400).sub(300).get();
     
     let total = num1.add(100);
     let total = num1.add(200);
     let total = num1.sub(500);
     let total=  num1.get();

     println!("Total:{}",total);
 }
 

 trait Math{
     type T; // asserted type in traits
     // type U;
     fn get(&self)->Self::T;
     fn add(&mut self,num:Self::T)-> Self::T;
     fn sub(&mut self,num:Self::T)-> Self::T;
 }
 
 #[derive(Debug)]
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
     type T = i32;
     fn get(&self)->Self::T{
         return self.num;
     }
     fn add(&mut self,num:Self::T)->Self::T{
         self.num+=num;
         return self.num;
     }
     fn sub(&mut self,num:Self::T)->Self::T{
         self.num-=num;
         return self.num;
     }
 }

 struct MyNumber(f64);

 impl MyNumber{
    fn new(num:f64)->Self{
        MyNumber(num)
    }
    fn default()->Self{
        MyNumber(0.0)
    }
}

impl Math for MyNumber{
    type T = f64;
    fn get(&self)->Self::T{
        return self.0;
    }
    fn add(&mut self,num:Self::T)->Self::T{
        self.0+=num;
        return self.0;
    }
    fn sub(&mut self,num:Self::T)->Self::T{
        self.0-=num;
        return self.0;
    }
}