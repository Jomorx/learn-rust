use std::fmt::Debug;
use std::fmt::Display;

use std::ops::Add;

#[derive(Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> Add<Point<T>> for Point<T>
where
    T: Copy + Display + Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<T> for Point<T> 
    where
        T: Copy + Add<Output = T>{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x:self.x+rhs,
            y:self.y+rhs
        }
    }
}

impl Add<Point<f64>> for Point<i32> 
{
    type Output = Point<i32>;
    fn add(self, rhs: Point<f64>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


impl<T> Debug for Point<T> where T:Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Point {{ x :{} y :{} }}",self.x,self.y)
    }
}

impl<T> Display for Point<T> where T:Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Point {{ x :{} y :{} }}",self.x,self.y)
    }
}
fn main() {
    let p1:Point<i32> = Point::new(1, 2);
    let p2 = Point::new(0.2, 0.4);
    println!("{}",p1+p2);
    println!("{:?}",p1+2_i32);

}
