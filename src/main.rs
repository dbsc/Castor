use std::{fs, result};
fn reader() -> Vec<u8> {
    let path = "/home/dbsc/litelab-crm/docker-compose.yml";
    let contents = fs::read(path).expect("can't read files");
    contents
}
fn crasher() -> Vec<u8> {
    let check = reader();
    let mut full: Vec<u8> = Vec::new();
    let kol = check.len();
    for i in 0..kol {
        let byte = check[i];
        if byte == 44 {
            break;
        }
        full.push(byte);
    }
    full
}
fn main() {
    let result = crasher();
    for byte in result {
        print!("{byte}");
    }
    println!();
}
