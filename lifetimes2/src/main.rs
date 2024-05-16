fn main() {
    println!("Hello, world!");
    let mut b= Box::new(100);
    println!("{}",b);
    square1(&mut b);
    println!("square1:{}",b);
    b = square2(b);
    println!("square2:{}",b);

    let b2 = square3(&mut b);

    square3(&mut b);

   

    let b2= Box::new("Hello");
    let b3 = Box::new(3.14);
    let b4: Box<bool>= Box::new(false);

    
}

fn square1<'b>(v: &'b mut Box<i32>){
    **v = **v * **v;
}

fn square2(mut v:Box<i32>)->Box<i32>{
    *v = *v * *v;
    return v;
}

fn square3<'b>(v: &'b mut Box<i32>)-> Box<i64>{
   //let k:Box<i64>= Box::new(**v as i64 * **v as i64); 

   let k:i64 = **v as i64 * **v as i64;
    //return k // problem of c and c++

    return Box::new(k);
}

struct A<'a>{
    s1:&'a str,
    s2:String,
    s3:&'a i32,
}

// 24 bytes A.s2[ptr,cap,length]

// str will be allocated only when the value is assigned to str