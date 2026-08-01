use rand::Rng;
use std::{fs, range};
fn start() {
    println!(
        r#"                     .d888888888888b.
                   .d8888888888888888b.
                  d888888888888888Y88888b.
                 d8888888888888888 " Y8888b
 .d8888888b.     88888888888888888    88888
d888888888888b..d888888888888888888  .8888P
Y888888888888888888888888888P  Y88888888P"
 "Y88888888P"   "Y88888888P"     │  │  │
                                 └─ .──┘
"#
    );
    println!("Alpha_0.3");
}
fn check_for_ten(num: u32) -> u32 {
    let mut add = 0;
    let remind = num % 10;
    if num == 0 {
        println!("ntodo");
    } else {
        add = 10 - remind;
    }
    add
}
fn reader() -> Vec<u8> {
    let path = "/home/dbsc/litelab-crm/docker-compose.yml";
    let contents = fs::read(path).expect("can't read files");
    contents
}
fn crasher() {
    let mut parts = Vec::new();
    let mut check = reader();
    let kol = check.len();
    let forten = check_for_ten(kol as u32);
    let mut rng = rand::thread_rng();
    check.extend((0..forten).map(|_| rng.gen_range(0..=255)));
    let size = check.len() / 10;
    let mut pieces = check.chunks_exact(size);
    for _ in 0..10 {
        parts.push(pieces.next().unwrap());
    }
    for (i, part) in parts.iter().enumerate() {
        println!("part{}: {:?}", i + 1, part);
    }
}
fn main() {
    start();
    crasher();
}
