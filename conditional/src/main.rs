fn main() {
    let number1= -5;
    if number1>=0{
        println!("number:{} is a positive number",number1);
    }else{
        println!("number:{} is a negative number",number1);
    }

    let number2= if true{
        println!("100");
         100
       // println!("{}",number2) ;
    }else{
        println!("200");
        200
    };

    println!("{:?}",number2);

    let gender: char='M';
    let age:u8=39;
    // true && true -->true
    if (gender=='M' || gender=='m') && age>=21{
        println!("He is eligible for marriage in India");
        // False && True --> False
    }else if (gender=='F' || gender=='f') && age>=18{
        println!("She is eligible for marriage in India");
    }else{
        println!("invalid data");
    }
}
