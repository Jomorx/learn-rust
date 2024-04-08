use std::fmt::Display;

#[derive(Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{} {}", self.x, self.y);
        f.write_str(&s)
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        // todo!()
        println!("drop x")
    }
}

fn main() {
    {
        let a = Point { x: 10, y: 10 };
        let b = a; // 这里发生了所有权 move，a 在后续不能使用了
        println!("{}", b)
    }
    println!("111")
}
