enum Color {
    RED(String),
    GREEN(String)
}

fn main(){
    choose_color(Color::RED(String::from("RED")));
    choose_color(Color::GREEN(String::from("GREEN")));
    choose_color(Color::GREEN(String::from("GREEN")));
}

fn choose_color(color:Color) {
    // match color {
    //     Color::RED(color) =>{
    //         println!("{color}")
    //     },
    //     Color::GREEN (color)=>{
    //         println!("{color}")
    //     }
    // }
    if let Color::RED(color) = color{
        println!("{color}")
    }else if let Color::GREEN(color) = color{
        println!("{color}")
    }else {
        println!("default")
    }
}