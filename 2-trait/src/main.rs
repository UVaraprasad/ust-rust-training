use core::ops::Add;
fn main() {

    let mut num1 = Number::new(100);
    let mut num2= Number::new(100.343);
    let mut str1= Number::new(Str("Hello".to_string()));


    //let mut num2= Number::new(true);

    let result = num1.add(100).add(200).add(300).get();
    
    println!("{}",result);

    let result = num2.add(101.12).add(102.34).get();
    println!("{}",result);

   // let result = str1.add(Str(" World".to_string())).get();

    }

   
    
    #[derive(Debug,Clone)]
    struct Str(String);

    impl Add for Str {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Self {
                // x: self.x + other.x,
                // y: self.y + other.y,

                0:self.0 + &other.0,
            }
        }
    }
    
    trait Math<T>{
        fn add(&mut self,num:T) -> &mut dyn Math<T>;
        fn get(&self)->T;
    }
    
    #[derive(Debug)]
    struct Number<T>(T);

    impl<T> Number<T>{
        fn new(num:T)->Self{
            Self(num)
        }
    }
    
    impl<T> Math<T> for Number<T> where T:std::ops::Add<Output = T>,T:Copy{
        fn add(&mut self,num:T) -> &mut dyn Math<T> {
            self.0 = self.0+num;
            return self;
        }
        fn get(&self)->T {
            return self.0;
        }
    }