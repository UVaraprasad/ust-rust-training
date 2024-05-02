fn main() {

    let mut k:i32 = 100; // stack allocated
    let mut kb:Box<i32> = Box::new(200); // heap allocated
    k+=1;
    *kb+=1;

   //let mut k2 = k;

    increment1(k);
    println!("k:{}",k);
    increment2(&mut kb);
    increment3(&mut k);
    printnum(&mut kb);
    println!("kb:{}",kb);
    println!("k:{}",k);
    // -----------
    {
    let k3= &mut k;
    *k3 =400;
    println!("k3:{}",*k3);
    }

    println!("k:{}",k);

    {
        let kb2:Box<i64>=Box::new(1231231); // heap allocated
    } // drop is applied by the rust compiler

}

fn increment1(mut num:i32){
    num+=1;
}

fn increment2( num:&mut Box<i32>){
    **num+=1;
}

fn increment3(num:&mut i32){
    *num+=1;
}


fn printnum(num: &mut Box<i32>){
    **num+=1;
    println!("The number is:{}",*num);
}

// cannot have two variables which are mutable to the same memory in the same scope