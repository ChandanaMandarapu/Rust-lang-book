fn main () {

    // case 1 

    // here what happens is we created a func takes_ownership and passes some string choose that arg now what happens s throws an error s cant be borrowed after moved remember s here is string right so what is prblm is after the scope ends soem string gets dropped

    let s : = String :: from ("hello");
    takes_ownership(some_string:s);
    println!("{}",s);

    // case 2 integer integers are copied so we can still use x after funccall 

    let x : i32 = 5;
    makes_copy(some_integer:x);
    println!("{}",x);

    // case 3 
    // here we have variable s1 that equals to the return value of funcn give ownership and that func returns the string returning the string moves the ownership of the string to the s1 variable and we can use it back hoo

    let s1 = gives_ownership();
    println!("s1 = {}",s1);

    // case 4 we can take ownership and give back

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy (some_integer : i32){
    println!("{}",some_integer);
}

fn gives_ownership () -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string : String) -> String {
    a_string
}