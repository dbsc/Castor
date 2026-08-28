use rand::Rng;
use std::fs;
use std::io;

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
    println!("Alpha_0.4");
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
    let final_path: String;

    println!("Do you want to change the file path? (y / n)  [ default (n) ]  ");
    let mut pathynyn = String::new();
    io::stdin()
        .read_line(&mut pathynyn)
        .expect("incorrect input, try Y or N");

    let answer = pathynyn.trim();

    if answer == "y" {
        println!("Write the path");
        let mut path = String::new();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read line");

        let trimmed_user = path.trim().to_string();

        if trimmed_user.is_empty() {
            final_path = String::from("/home/nut/Castor/src/main.rs");
        } else {
            final_path = trimmed_user;
        }
    } else {
        final_path = String::from("/home/nut/Castor/src/main.rs");
    }

    let contents = fs::read(&final_path)
        .expect(&format!("can't read file at: {}", final_path));
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

