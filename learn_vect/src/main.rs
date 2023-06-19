use std::collections::HashMap;

fn main() {
    let color = vec![String::from("red"), String::from("green")];
    let score = vec![1, 2];
    let map: HashMap<_, _> = color.iter().zip(score.iter()).map(|(k, v)| (k.clone(), v + 1)).collect();
    for  v in score.iter() {
        println!("{}",v)
    }
    
}