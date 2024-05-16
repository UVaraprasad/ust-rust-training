fn main() {
    let s1=String::from("hello World");
    let s2 = String::from("hello universe");
    //println!("{}",x);
    println!("Length of string1:{}",length(&s1));
    println!("{}",s1);
    let x: i32 = 10;
    {
        let y = x;
        let z = 100;
        {
            let z1 = 101;
        }
        let s3: &String=&s1; // borrowing. Shallow copy
        let s4 = s1.clone(); // deep copy
        let s5 = s1.as_str();
        }
    println!("Length of string2:{}",length(&s2));
    println!("{}",x);
    let s3 = "hello".to_string();
    let s4 = "hello world".to_string();
    let s6=longest("hello my world","hello world");
    let s5=longest(&s3, &s4);
}

fn length<'a>(str1:&'a str)->i32{  
    return str1.len() as i32; // s1 would not be deallocated
} // do not deallocate s1 but give back the ownership to the originnal varialbe

fn length1(s1:&String)->i32{
    return s1.len() as i32;
}

fn longest<'a>(s1:&'a str,s2:&'a str)->&'a str{
    if s1.len()>s2.len(){
        return s1
    }else{
        return s2
    }
}



// fn longest2(s1:&str,s2:&str)->&str{
//     if s1.len()>s2.len(){
//         return s1
//     }else{
//         return s2
//     }
// }