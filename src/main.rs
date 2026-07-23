use std::fs;
fn main() {
    let path = "/home/dbsc/disk/text.txt";
    let contents = fs::read(path).expect("can't read files");
    println!("{:?}", contents);
}
