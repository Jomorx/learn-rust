use std::collections::HashMap;
#[test]
fn main() {
    let res = count_world(&["hello", "world"]);
    println!("{:?}", res)
}
fn count_world(input: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    input.iter()
    .flat_map(|s| s.chars())
    .for_each(|c| *map.entry(c).or_default()+=1 );

    map
}
