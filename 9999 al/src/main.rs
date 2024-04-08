#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl From<(i32, i32)> for Point {    // 实现从 (i32, i32) 到 Point 的转换
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}
impl From<[i32; 2]> for Point {      // 实现从 [i32; 2] 到 Point 的转换
    fn from([x, y]: [i32; 2]) -> Self {
        Point { x, y }
    }
}
fn example() {
    // 使用from()转换不同类型
    let origin1 = Point::from((0, 0));
    let origin2 = Point::from([0, 0]);
    // 使用into()转换不同类型
    let origin3: Point = (0, 0).into();
    let origin4: Point = [0, 0].into();
    println!("{:?} {:?} {:?} {:?}",origin1,origin2,origin3,origin4)
}


fn main(){
    example()
}