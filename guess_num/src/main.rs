use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("猜数！");
    let secret_num = rand::thread_rng().gen_range(1,100);
    println!("secret_num is {}",secret_num);
    loop {

        let mut guess_num = String::new();
    
        io::stdin().read_line(& mut  guess_num).expect("无法读取行");
        let guess_num:u32 =match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
            println!("输错了");
            continue;
        },
        }; 
    
        match guess_num.cmp(& secret_num){
            Ordering::Less => println!("小了"),
            Ordering::Equal => {
                println!("猜中了");
                break;
            },
            Ordering::Greater => println!("大了"),
        }
        println!("你猜的数是{}",guess_num);
    }
}
