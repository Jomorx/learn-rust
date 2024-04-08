fn main() {
    let some_n = Some(123);
    let some_n2 = some_n.map(|c| c * 2);
    // 和 map 用法一样，只不过 map 会自动将返回值用 Some 包起来
    // 而 and_then 则需要手动这么做
    let some_n3 = some_n.and_then(|c| Some(c * 2));
    println!("{:?}", some_n);  // Some(123)
    println!("{:?}", some_n2);  // Some(246)
    println!("{:?}", some_n3);  // Some(246)
}
