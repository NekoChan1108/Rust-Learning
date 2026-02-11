use std::fs::File;
use std::io;
use std::io::Read;
#[allow(unused)]
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", read_file("Cargo.toml"));
    File::open("Cargo.toml")?;
    Ok(())
}
