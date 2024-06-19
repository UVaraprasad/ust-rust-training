fn main() {
    let f1 = || {
        println!("Hello World");
    };

    //f1();

    let a = 100;
    //
    let add1 = |x: i32, y: i32| -> i32 {
        f1();
        x + y + a
    }; // this is not an anpnypous function , this is just clousure

    let a1 = add1(10, 200);

    println!("add through a clousure:{}", a1);

    let k = |x: i32, y: i32| -> i32 { x + y }(10, 20); // this is anonymous function
    {
        let mut kp = Box::<i32>::new(100);
        let mut s1 = "hello".to_string();
        let mut s2 = "hello".to_string();

        let mut display = move || {
            *kp= 200;
            s1.push_str("hello world");
            println!("{:?}", kp);
            //return s1;
            s2;
        };

        display();
        println!("{}",s2);
        //println!("{}",s1);
        //println!("{:?}", kp);
    }
}

fn add2(x: i32, y: i32) -> i32 {
    x + y
}
