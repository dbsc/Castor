use rand::Rng;
use std::fs;
use std::io;
use reed_solomon_erasure::galois_8::ReedSolomon;

fn start() {
    println!("Alpha_0.5");
}

fn check_for_allig(num: usize, pieces_count: usize) -> usize {
    let add;
    let remind = num % pieces_count;
    if num == 0 {
        println!("ntodo");
        add = 0;
    }
    else if remind == 0 {
        add = 0;
    }
        else {
        add = pieces_count - remind;
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
        println!("Etner the path");
        let mut path = String::new();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read path");
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
    println!("Enter pieces count");
    let mut chunks_input = String::new();
    io::stdin()
        .read_line(&mut chunks_input)
        .expect("Failed to read a count");

    let pieces_count: usize = chunks_input.trim().parse().unwrap_or(2).max(2);
    let parity_count = (pieces_count + 1) / 2;
    let mut check = reader();
    let forten = check_for_allig(check.len(), pieces_count);
    let mut rng = rand::thread_rng();
    check.extend((0..forten).map(|_| rng.gen_range(0..=255)));
    if forten > 0 {
        if let Some(last_byte) = check.last_mut() {
            *last_byte = forten as u8;
        }
    }
    let size = check.len() / pieces_count;
    let mut master_space = Vec::new();
    let mut pieces = check.chunks_exact(size);
        for _ in 0..pieces_count {
        master_space.push(pieces.next().unwrap().to_vec());
    }

    for _ in 0..parity_count {
        master_space.push(vec![0u8; size]);
    }
    let r = ReedSolomon::new(pieces_count, parity_count).unwrap();
    r.encode(&mut master_space).unwrap();
        println!("Do you want to change the output directory? (y / n)  [ default (/home/nut/2Castor/) ]");
    let mut dir_choice = String::new();
    io::stdin().read_line(&mut dir_choice).expect("Failed to read choice");

    let final_dir = if dir_choice.trim() == "y" {
        println!("Write the directory path:");
        let mut user_dir = String::new();
        io::stdin().read_line(&mut user_dir).expect("Failed to read directory");
        let trimmed_dir = user_dir.trim().to_string();
        if trimmed_dir.is_empty() {
            String::from("/home/nut/2Castor/")
        } else {
            if trimmed_dir.ends_with('/') { trimmed_dir } else { format!("{}/", trimmed_dir) }
        }
    } else {
        String::from("/home/nut/2Castor/")
    };

    for (i, part) in master_space.iter().enumerate() {
        let filename = format!("{}part{}.dat", final_dir, i + 1);
        fs::write(&filename, part).expect("Failed to write file");
        println!("Saved: {}", filename);
    }

}

fn main() {
    start();
    crasher();
}

