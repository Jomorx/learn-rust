use std::io;

fn main() {
    println!("猜数！");
    let mut guess_num = String::new();
    io::stdin().read_line(& mut  guess_num).expect("无法读取行");
    println!("你猜的数是{}",guess_num);
}
