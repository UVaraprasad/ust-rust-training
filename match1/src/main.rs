fn main() {
    let day = 4;

    match day{
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Turesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("no day"),
    }

    let number = 12;
    let multiple = 3;

    match number{
        0 => println!("number is zero"),
        1 | 2 => println!("number is 1 or 2"), 
        n if n%multiple==0 => println!("number is 3 multiple:{}",n),
        _ => println!("do nothing"),
    }
    // epplore more on match pattern .. 
}


// task 
// Take a number 
// check whether the number is multiple of 2,4 or 8 
// it should apply multiple patterns..

/*

int number = 8;
switch number{
    case number%8==0:
    print("number is multiple of 8");
    case number%4==0;
    print("number is multiple of 4");
    case number%2==0:
    print("number is multiple of 2");
    case number%3==0:
    println("number is multiple of 3");
}
*/
