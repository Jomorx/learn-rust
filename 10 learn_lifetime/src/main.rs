
fn main() {
    let res = longest("111", "22223");
    println!("{res}")
}

fn longest<'a>(str1:& 'a str,str2:& 'a str)-> & 'a str{
    if str1.len()>str2.len() {
        return str1
    }else {
        return str2
    }
}