fn main() {
    let foo = 1;
    let bar = ||{
        println!("{foo}")
    };
    bar()
}
