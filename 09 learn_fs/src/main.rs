use std::{
    fs::File,
    io::{self, ErrorKind},
};

fn main() {
    let s = read_file();
    
}
fn read_file() -> Result<String, io::Error> {
    let f: Result<File, io::Error> = File::open("hello.txt");
    let f = match  f {
        Ok(result) => {
            Ok(result)
        },
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("{:?}",fc)
                },
                Err(e) => Err(e),
            },
            otherError=>panic!("{:?}",otherError)
            
        },
    }

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
