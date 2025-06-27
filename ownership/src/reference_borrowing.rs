fn main () {
    let s1 = String::from("hello");
    let (s2,len) =  caluclate_length(s1);
    println!("length of {}",s2,len);
}

fn caluclate_length(s:String) -> (String,usize) {
    let length = s.len(); //len() returns length of stirng
    (s,length);

}