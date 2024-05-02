fn main() {

    let mut person = ("Jiten",38);
    println!("name of a person:{}",person.0);
    println!("age of a person:{}",person.1);

    println!("Person:{:?}",person); // 

    person.1 = 48; // Mutation
    person.0="Jiten P";

    let (mut name,mut age)=person; // destruction of tuple

    println!("name of the person:{}",name);

    age = 48;

    println!("age of the person:{}",age);

    let (name1,age1)=("Jiten",39);

    let name2 = person.0;

    //let name2,age2 = "Jiten",42;

    // let mut i:i64 = 1003423423;

    // i = 12312213;

    let student:(&str,u8,bool)=("Jiten",38,true);

}

// Rust does not support type inferencing


//std::fmt::Display --> Trait 

// &str --> String slice
// UTF-8
// 1-4 bytes
// 

/*
Tuple 
String
If Else
Loops
Ownership
Match
Mut
*/