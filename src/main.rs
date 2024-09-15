use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;
use hex;
use std::time::Instant;
use crc64;
use std::collections::HashMap;

// Calculate sha256 from file
fn sha256_from_file(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();
    hex::encode(hash)
}

// Calculate CRC64 from file
fn crc64_from_file(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();


    let cksum = crc64::crc64(0, &*buffer);
    format!("{:x}", cksum)
}


fn main() {

    // Hashmap with the crc and list of files
    let mut crc_map: HashMap<String, Vec<String>> = HashMap::new();
    let path = ".";
    let now = Instant::now();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            continue;
        }

        let file_path = entry.path().to_str().unwrap();

        let crc64 = crc64_from_file(file_path);
        crc_map.entry(crc64).or_insert(Vec::new()).push(file_path.to_string());

    }

    let elapsed = now.elapsed();

    // print the crc and the list of files
    for (crc, files) in crc_map.iter() {
        println!("CRC64: {}", crc);
        for file in files {
            println!("{}", file);
        }
        println!("\n");
    }

    println!("Elapsed: {:.2?}\n", elapsed);
}
