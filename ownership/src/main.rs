 fn main() {
    println!("Hello, world!");

    //  how stack and heap works
    fn a () {
    let x : &str = "chandu";
    let y : i32 = 99;
    b();


    fn b() {
        let x  = String::from("mandapru");
    }
}

// memory allocation and copying values 
// do u think when when u wanna store s1 value into s2 maybe pointer gets changed and again a new heap space will be created for s2 but no that will be another big costly thing another thing we would expect is shallow copying like showing both as same but both doesnt happen in rust if u want to store s1 value in s2 rust invalidates the value of s1 that means it completly removes and now if u want to copy u can use a method called .clone()

let x : i32 = 8;
let y : i32 = x; // copy
// here rust does copy doesnt move rust has a copy trait for fixed datatypes which are stored in stack 

let s1 = String::from("chandu");
let s2 = s1.clone();

println!("{}", s2);

}


// 1. each value in rust has a variable thats called its owner .
// 2. there can only be one owner at a time.
// 3. when the ownere goes out of scope the value is dropped.

// ownership in functions

