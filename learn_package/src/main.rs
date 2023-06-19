mod get_thing {
    pub mod things {
        use rand::Rng;
        pub fn get_random() -> i32 {
            return 111;
        }
        pub fn get_time() -> i32 {
            return rand::thread_rng().gen_range(1, 100);
        }
    }
}
use crate::get_thing::things::{get_random, get_time};
fn main() {
    println!("{}", get_random());
    println!("{}", get_time());
}
