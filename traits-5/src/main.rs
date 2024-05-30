fn main() {
    let mut r1 = Number::new(100);
    println!("r1:{:?}",r1);
    let sum = r1.add(100).add(200).get();
    println!("Sum:{}",sum);
}

trait Math {
    fn add(&mut self, num: i32) -> &mut dyn Math;
    fn get(&mut self) -> i32;
}

#[derive(Debug)]
struct Number(i32);

impl Number {
    fn new(num: i32) -> Self {
        return Number(num);
    }
}

impl Math for Number {
    fn add(&mut self, num: i32) -> &mut dyn Math {
        self.0 = self.0 + num;
        return self;
    }
    fn get(&mut self) -> i32 {
        return self.0;
    }
}
