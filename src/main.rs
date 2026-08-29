use rand::Rng;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use reed_solomon_erasure::galois_8::ReedSolomon;
use sha2::{Sha256, Digest};


fn start() {
    println!("Alpha_0.6");
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
fn reader() -> (Vec<u8>, String) {

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
    (contents, final_path)
}

fn crasher() {
    println!("--- Castor Crasher ---");
    println!("Enter pieces count");
    let mut chunks_input = String::new();
    io::stdin()
        .read_line(&mut chunks_input)
        .expect("Failed to read a count");

    let pieces_count: usize = chunks_input.trim().parse().unwrap_or(2).max(2);
    let parity_count = (pieces_count + 1) / 2;
    let (mut check, origin_path) = reader();
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

    let mut chunk_hashes = Vec::new();

    for part in master_space.iter() {
        let mut hasher = Sha256::new();
        hasher.update(part);
        let hash_result = hasher.finalize();
        let hash_string = format!("{:x}", hash_result);

        chunk_hashes.push(hash_string.clone());

        let filename = format!("{}{}", final_dir, hash_string);
        fs::write(&filename, part).expect("Failed to write file");
        println!("Saved chunk: {}", filename);
    }
    let mut meta_content = format!("{}\n{}\n", pieces_count, parity_count);

    let hashes_line = chunk_hashes.join(",");
    meta_content.push_str(&hashes_line);
    let mut manifest_hasher = Sha256::new();
    manifest_hasher.update(meta_content.as_bytes());
    let manifest_hash = format!("{:x}", manifest_hasher.finalize());
    let manifest_path = format!("{}{}.manifest", final_dir, manifest_hash);
    fs::write(&manifest_path, &meta_content).expect("Failed to write manifest");
    println!("Manifest hash ID: {}", manifest_hash);
    println!("Enter a name for this file:");
    let mut alias_input = String::new();
    io::stdin().read_line(&mut alias_input).expect("Failed to read name");
 let alias = alias_input.trim();
    let file_name = origin_path.split('/').last().unwrap_or("file.dat");
    let link_line = format!("{}_{} = {}\n", alias, file_name, manifest_hash);

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("links.txt")
    .expect("Failed to open links.txt");

    file.write_all(link_line.as_bytes()).expect("Failed to write to links.txt");
}
fn restorer() {
    println!("--- Castor Restorer ---");
     let links_content = fs::read_to_string("links.txt")
        .expect("Failed to read links.txt");
    println!("Your saved files:\n{}", links_content);
    println!("Copy and paste the manifest hash ID you want to restore:");
    let mut manifest_hash = String::new();
    io::stdin()
        .read_line(&mut manifest_hash)
        .expect("Failed to read hash");

    let clean_hash = manifest_hash.trim();
    println!("You chose hash: {}", clean_hash);
    let manifest_path = format!("/home/nut/2Castor/{}.manifest", clean_hash);
    let manifest_content = fs::read_to_string(&manifest_path)
        .expect("Failed to read manifest file!");


    let mut lines = manifest_content.lines();

    let pieces_count: usize = lines.next().unwrap().parse().expect("Invalid N");
    let parity_count: usize = lines.next().unwrap().parse().expect("Invalid M");

    println!("Success! Matrix loaded: N = {}, M = {}", pieces_count, parity_count);
    let hashes_raw = lines.next().unwrap();
    let chunk_hashes: Vec<&str> = hashes_raw.split(',').collect();

    println!("Total chunks to find: {}", chunk_hashes.len());
    let mut master_space = Vec::new();
    let mut alive_count = 0;

    for hash in chunk_hashes.iter() {
        let chunk_path = format!("/home/nut/2Castor/{}", hash);

        let file_result = fs::read(&chunk_path);

        if file_result.is_ok() {
            let bytes = file_result.unwrap();
            master_space.push(Some(bytes));
            alive_count = alive_count + 1;
        } else {
            master_space.push(None);
        }
    }

    println!("Alive chunks: {} / {}", alive_count, chunk_hashes.len());
    if alive_count < pieces_count {
        println!("Error: Too many chunks lost! Cannot restore file. :(");
        return;
    }

    let r = ReedSolomon::new(pieces_count, parity_count).unwrap();

    r.reconstruct(&mut master_space).unwrap();

    println!("good! :)");
        let mut final_file = Vec::new();
    for i in 0..pieces_count {
        let chunk_bytes = (&master_space[i]).clone().unwrap();
        final_file.extend(chunk_bytes);
    }

    let last_byte = final_file.pop().unwrap();
    let padding_len = last_byte as usize;
    for _ in 0..(padding_len - 1) {
        final_file.pop();
    }

    let mut file_name = String::from("restored_file.dat");
    let links_content = fs::read_to_string("links.txt").expect("Failed to read links.txt");

    for line in links_content.lines() {
        if line.contains(clean_hash) {
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts.len() == 2 {
                file_name = parts[0].to_string();
            }
        }
    }

    fs::write(&file_name, final_file).expect("Failed to save file");
    println!("DONE! File restored successfully as '{}'!", file_name);


}



fn main() {
    start();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:\n  castor crash\n  castor restore");
        return;
    }
    let command = args[1].trim();
    if command == "crash" {
        crasher();
    } else if command == "restore" {
        restorer();
    } else {
        println!("Unknown command: '{}'. Use 'crash' or 'restore'.", command);
    }
}


