fn main() {
    let s1 = String::from("hello ");
    let mut s2 = String::from("world");
    let s3 = s1+& s2;
    s2 = String::from("123");
    println!("{s3}")
}
