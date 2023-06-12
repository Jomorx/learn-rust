
fn main() {
    let str1 = String::from("11111");
    let str2 = check_msg(str1);
    let str3 = "111111";
    let str4 = str3;
    println!("str2: {},str3: {} ,str4:{}",str2,str3,str4);
}


fn check_msg (s1:String)->String {
    println!("s1: {}",s1);
    s1
}