fn main() {
    let mut r1 = Number1::new(100);
    println!("r1:{:?}",r1);
    let sum = r1.add(100).add(200).get();
    println!("Sum:{}",sum);

    if sum>0{
        panic!("I dont like this sum");
    }
    
    let mut n2 = Number2::<f32>::new(Some(100.25));

    match n2{
        Ok(mut v)=>{
          let result=  v.add(12.123).add(13.45).get();
          println!("What v is ,{:?}",v);
        },
        Err(e)=>println!("{e}"),
    }

    let mut n3 = Number2::<f32>::new(Some(123.123)).expect("no value is given");

    let mut n4 = Number2::<f32>::new(None).unwrap();


}

trait Math {
    type Item;
    fn add(&mut self, num: Self::Item) -> &mut dyn Math<Item = Self::Item>;
    fn get(&self) -> Self::Item;
}



#[derive(Debug)]
struct Number1(i32);

impl Number1 {
    fn new(num: i32) -> Self {
        return Number1(num);
    }
}

impl Math for Number1 {
    type Item=i32;
    fn add(&mut self, num: i32) -> &mut dyn Math<Item=i32> {
        self.0 = self.0 + num;
        return self;
    }
    fn get(&self) -> i32 {
        return self.0;
    }
}


#[derive(Debug)]
struct Number2<T>(T);

impl<T> Number2<T> {
    fn new(num: Option<T>) -> Result<Self,String> {
        match num{
            Some(n)=> {
                Ok(Number2(n))
            },
            None=>{
                Err("Nothing is given".to_string())
            }
        }
    }
}

//impl<T: std::ops::Add<Output = T>+Copy> Math for Number2<T> {
    impl<T> Math for Number2<T> 
                where T: std::ops::Add<Output = T>,
                T: Copy {
    type Item=T;
    fn add(&mut self, num: T) -> &mut dyn Math<Item=T> {
        self.0 = self.0 + num;
        return self;
    }
    fn get(&self) -> T {
        return self.0;
    }
}


// struct Point{
//     l:f32,
//     b:f32
// }

// let p1 = Point{l:10.o,b:12.0};
// let p2= Point{l:10.o,b:12.0};
// let p3 = p1+p2;
