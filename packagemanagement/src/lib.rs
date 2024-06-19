pub fn greet(){
    println!("Hello World from lib");
}

pub mod shape;

// there must be lib.rs file
// other than main.rs or the file that has a main fn , is treated as a module
// thre are two types of pacakge management in rust
// old way and new way.
// mod.rs but in latest way there is no need of mod.rs file

// each .rs file is a module (dont consider main.rs)