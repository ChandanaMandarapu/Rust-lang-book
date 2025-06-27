fn main() {

    // calling funcns
    another_func( 9 ,  99);
    my_func();

    // mutability 
    let mut x : i32 = 5;
    println!("value of x is {}",x);
    x = 9;
    println!("value of x is {}",x);

    const AVG_MARKS : u32 = 1000;
    println!("value of avg marks is {}",AVG_MARKS);

    // shadowing
    // first y is shadowed to y variable both are mutable and we can change the types
    let y : i32 = 4;
    println!("value of y is {}",y);
    let y : bool = true;
    println!("value of y is {}",y);

    // scalar datatypes 

    // Integers 

    let a : i32 = 98_222; //decimal
    let b : i32 = 0xff;
    let c : i32 = 0o77; 
    let d : i32 = 0b111_000;
    let e : u8 = b'A';
    let sum : i32 = 9 + 9;
    // floating
    let diff : f64 = 98.8 - 34.5;

    // bool and characters

    let t : bool = true;
    let h : bool = false;

    let k : char = 'z';
    let v : char = '😁';

    // compound dataypes - tuples
    // tuples - a group of values fixed sized array kind maybe with diff data

    let tup :(&str,i32) = ("chandana",21);
    // accessing elements frm tuples
    // destructuring
    let (name, age) = tup;
    // by dot notation
    let age : i32 = tup.1;

    // arrays

    let marks : [i32;4] = [23,45,66,77];
    // arrays of are fixed type
    let failed_marks = marks[1];

    // functions

    fn my_func() {
        println!("my function");
    }
    fn another_func(x:i32, y:i32) -> i32{
        println!("value of x is {}",x);
        println!("value of y is {}",y);
        let sum : i32 = x + y;
        return sum; 
    }
    // statements and expressions
    // my_func is a statement as doesnt return anything
    // another func is a expression as returns smtng

    // control flow

    
    let marks = 75;

    if marks >= 90 {
        println!("You got an A+!");
    } else if marks >= 75 {
        println!("You got a B+!");
    } else if marks >= 50 {
        println!("You passed!");
    } else {
        println!("You failed.");
    }


    // loops
    let mut counter : i32 = 0;
    let result : i32 = 
    loop{
        counter += 1;
        if counter == 10 {
            break counter;
        }
    }
    println!("result is {}",result);

    // while loop

    let mut num : i32 = 9;

    while num ! = 0 {
        println!("{}",num);

        number -= 1;

    }

    println!("stop it");

    // for loops

    let n : [i32 :] = [12,44,66,77];

    for elt in n . iter () {
        println!("value is {}",elt);
    }

    for number in 1..9{
        println!("{}",number);
    }
}
