

// this file is for reading from the file in rust

use std::fs::File;
use std::io::{self, BufRead};

pub fn searchforFile(){
    println!("Enter the file name to search for:");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();
    match File::open(file_name) {
        Ok(file) => {
            // if the file exists, read its contents, if not throw error
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(content) => println!("{}", content),
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),

            
        }
    }




}