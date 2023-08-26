macro_rules! sum {
    ($($num: expr);+) => {
        0$(+ $num)*
    };
    ()=>{
        1000
    }
}
#[test]
fn main(){
    let res =sum!(1;2;3;4);
    assert_eq!(10,res);
    assert_eq!(1000,sum!())
}


#[test]
fn foo(){
    let s1 = String::from("哈哈哈哈");
    let s2 = String::from("222");
    let res = greater_than(&s1, &s2);
    let v = vec![1,2,3];
    let vv:Vec<i32> = v.iter().map(|item|{return  item*2}).collect();
    println!("{vv:?}");
    println!("{res}")
}

fn greater_than<'a>(s1:&'a String,s2:&'a String)->&'a String{
    let count = s1.len();
    println!("{count}");
    return  s1;
}