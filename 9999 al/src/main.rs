mod isPalindrome;
mod macro_test;

use std::{collections::HashMap, vec};

fn main() {
    let _res = two_sum(vec![5,4,6,2], 0);
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (index,item) in nums.iter().enumerate() {
       if let Some(val) = map.get(item) {
           return vec![index as i32,*val as i32]
       }else {
           map.insert(target-item, index as i32);
       }
    }
    vec![-1,-1]
}
