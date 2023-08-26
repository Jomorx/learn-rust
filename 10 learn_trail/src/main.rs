use learn_trail::{WeiBo, Publish};



fn main(){
    let wei_bo = WeiBo{
        author:String::from("111"),
        article_content:String::from("111")
    };
    println!("{}",wei_bo.publish_article());
}