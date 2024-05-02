fn main() {
  let str4="hello World!";
   println!("Length:{}",len(str4));

   println!("Length:{}",len("Hello World! How are you doing!"));

   #[warn(unused_variables)]
   let s1=reverse(str4);

let mytouple = (10,"Hello",1232.2131,true);

println!("{:?}",mytouple);

let mut k:Option<i32>=Some(1231);

match k{
    

}

}


fn len(s:&str)->u8{
    let mut count=0;
    for c in s.chars(){
        count= count +1;
    }
     count
}


fn reverse(s:&str)->String{
    let mut s1=String::new();
    for ch in s.chars(){
        s1 = ch.to_string()+&s1;
    }
    //println!("{}",s1);
    return s1;
}