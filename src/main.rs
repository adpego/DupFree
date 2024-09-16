use walkdir::{DirEntry, WalkDir};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use crc64;
use std::collections::HashMap;
use human_bytes::human_bytes;

struct FileInfo {
    file_path: String,
    size: u64,
}

// Calculate CRC64 from file
fn crc64_from_file(file_path: &str) -> String {
    println!("Processing: {}", file_path);
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();


    let cksum = crc64::crc64(0, &*buffer);
    format!("{:x}", cksum)
}

fn has_allowed_extension(entry: &DirEntry) -> bool {
    let allowed_extensions = vec!["jpg", "jpeg", "png", "pdf"];
    entry.path().extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| allowed_extensions.contains(&ext))
        .unwrap_or(false)  // If no extension, return false
}

fn main() {

    // Hashmap with the crc and list of files
    let mut crc_map: HashMap<String, Vec<FileInfo>> = HashMap::new();
    let path = "/home/adpego/Documents";
    let now = Instant::now();
    let mut total_size = 0;
    let mut reduced_size = 0;
    let mut num_files = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()).filter(|entry| entry.file_type().is_file()).filter(|entry| has_allowed_extension(entry)) {

        let file_path = entry.path().to_str().unwrap();

        let crc64 = crc64_from_file(file_path);
        let file_info = FileInfo {
            file_path: file_path.to_string(),
            size: entry.metadata().unwrap().len(),
        };
        total_size += file_info.size;

        num_files += 1;

        crc_map.entry(crc64).or_insert(Vec::new()).push(file_info);

    }

    let elapsed = now.elapsed();

    // print the crc and the list of files
    for (crc, files) in crc_map.iter() {
        reduced_size += files[0].size;
        if files.len() == 1 {
            continue;
        }
        println!("CRC64: {}", crc);
        if files.len() > 1 {
            println!("Files:");
        }
        for file in files {
            println!("{} {}", file.file_path, human_bytes(file.size as f64));
        }
        println!("\n");
    }

    println!("Total size: {}", human_bytes(total_size as f64));
    println!("Reduced size: {}", human_bytes(reduced_size as f64));
    println!("Size saved: {}", human_bytes((total_size - reduced_size) as f64));
    println!("Number of files: {}", num_files);
    println!("\nElapsed: {:.2?}\n", elapsed);
}
