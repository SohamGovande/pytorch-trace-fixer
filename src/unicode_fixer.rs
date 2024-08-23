use std::{env, process};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use indicatif::ProgressBar;

pub fn fix_invalid_utf8(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let suffix = "-fixed.json";
    if file_path.ends_with(suffix) {
        return Err("File path already ends with -fixed.json. Please pass a raw file into `fix_invalid_utf8`.".into());
    }
    let in_file = File::open(file_path).expect("Could not open file.");
    let in_reader = BufReader::new(in_file);
    let metadata = std::fs::metadata(file_path).expect("Could not get metadata");
    let file_size = metadata.len();
    println!("Filtering out {} bytes", file_size);

    let file_path_no_extension = &file_path[0..file_path.len() - ".json".len()];
    let out_file_path = String::from(file_path_no_extension) + suffix;
    let out_file = File::create(&out_file_path).expect("Unable to create file");
    let mut out_writer = BufWriter::new(out_file);
    let mut i = 0_u32;
    let bar = ProgressBar::new(file_size);
    let mut log = String::from("");

    for line_bytes in in_reader.split(b'\n') {
        i += 1;

        let line_bytes_vec = line_bytes.expect("Invalid line bytes");
        let line = String::from_utf8_lossy(&line_bytes_vec);

        let filtered_line: String = line
            .chars()
            .map(|c| {
                if c.is_ascii() && !c.is_ascii_control() {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        if line != filtered_line {
            let filter_log = format!("Filtered line {}: {} to {}", i, line, filtered_line);
            log.push_str(&filter_log);
        }

        writeln!(out_writer, "{}", filtered_line).expect("Error writing to putput");

        bar.inc((filtered_line.bytes().len() + 1) as u64);
    }
    bar.finish();
    if log.len() > 0 {
        println!("{}", log);
    }
    std::fs::remove_file(file_path).expect("Unable to delete original file");
    std::fs::rename(out_file_path, file_path).expect("Unable to rename fixed file");
    Ok(())
}

pub fn unicode_fixer_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected 1 argument: filepath of file to fix.");
        process::exit(1);
    }

    let file_path = &args[1];
    let result = fix_invalid_utf8(file_path);
    match result {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}