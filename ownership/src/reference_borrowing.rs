fn main () {
    let s1 = String::from("hello");
    let len =  caluclate_length(s1);
    println!("length of {}",s2,len);
}

// s takes the reference of string s1 rather than ownership it points to s1 and s1 is pointer in heap stored references are immutable by default 

// passing in refernces as func parameteres is called borrowing 
fn caluclate_length(s: &String) -> usize {
    let length = s.len(); //len() returns length of stirng
    length;
}