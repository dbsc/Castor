use rand::Rng;
use std::fs;
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
    println!("Alpha_0.2");
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
    let mut check = reader();
    let kol = check.len();
    let forten = check_for_ten(kol as u32);
    let mut rng = rand::thread_rng();
    check.extend((0..forten).map(|_| rng.gen_range(0..=255)));
    let size = check.len() / 10;
    let mut pieces = check.chunks_exact(size);
    let part1 = pieces.next().unwrap();
    let part2 = pieces.next().unwrap();
    let part3 = pieces.next().unwrap();
    let part4 = pieces.next().unwrap();
    let part5 = pieces.next().unwrap();
    let part6 = pieces.next().unwrap();
    let part7 = pieces.next().unwrap();
    let part8 = pieces.next().unwrap();
    let part9 = pieces.next().unwrap();
    let part10 = pieces.next().unwrap();
    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
    println!("part3: {:?}", part3);
    println!("part4: {:?}", part4);
    println!("part5: {:?}", part5);
    println!("part6: {:?}", part6);
    println!("part7: {:?}", part7);
    println!("part8: {:?}", part8);
    println!("part9: {:?}", part9);
    println!("part10: {:?}", part10);
}
fn main() {
    start();
    crasher();
}
