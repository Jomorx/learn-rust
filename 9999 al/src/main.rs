#[derive(Debug)]
struct Point {
    x:i32,
    y:i32
}

impl From<[i32; 2]> for Point {      // 实现从 [i32; 2] 到 Point 的转换
    fn from([x, y]: [i32; 2]) -> Self {
        Point { x, y }
    }
}
fn main() {
    let p1 = Point::from([1,2]);
    let p2= Point::from([1,2]);

    let mut list:Vec<_> = Vec::new();
    list.push(p1);
    list.push(p2);
    let mut list2 = list.into_iter()
    .map(|point| (Point::from([point.x,point.y])))
    .collect::<Vec<Point>>();
    println!("{:?}",list)
} 
