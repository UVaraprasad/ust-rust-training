fn main() {

    // let mut num1= Number::new(100);
     //let mut num1 = Number::<i32>{num:100};
//      let mut num2 = Number::<f32>{num:25.45};
//    //  let total: <dyn Math as Math>::T = num1.add(100);//.add(200).sub(100).add(300).add(400).sub(300).get();
     
//      let total = num1.add(100);
//      let total = num1.add(200);
//      let total = num1.sub(500);
//      let total=  num1.get();

    // println!("Total:{}",total);

    let s1:String = String::from("Hello world");
    let s2 = "Hey";
    let s3 = s1 + s2+ "Hey";
    println!("{} {}",s2,s3);
 }
 

 trait Math<T>{
     fn get(&self)->T;
     fn add(&mut self,num:T)-> T;
     fn sub(&mut self,num:T)-> T;
 }
 
 #[derive(Debug)]
 struct Number<T>{
     num:T
 }
 
 impl<T> Number<T>{
    
 }
 
 impl<T> Math<T> for Number<T>{
     fn get(&self)->T{
         return self.num;
     }
     fn add(&mut self,num:T)->T{
         //self.num = self.num+num;
         return self.num;
     }
     fn sub(&mut self,num:T)->T{
        // self.num=self.num-num;
         return self.num;
     }
 }
