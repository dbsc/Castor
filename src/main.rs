use std::fs;
fn reader() -> Vec<u8> {
    let path = "/home/dbsc/litelab-crm/docker-compose.yml";
    let contents = fs::read(path).expect("can't read files");
    contents
}
fn crasher() -> String {
    let check = reader();
    let mut full = String::new();
    let kol = check.len();
    for i in 0..kol {
        let num_toas = check[i].to_string();
        full.push_str(&num_toas);
    }
    full
}
fn main() {
    let rusult = crasher();
    println!("{rusult:?}");
}
