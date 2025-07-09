// this is to delete the file
use std::fs;
use std::io::{self, Write};
use std::fs::File;
use std::io::Write;

pub fn searchforFile() {
    fileReading::searchforFile();
}
pub fn delete_file() -> io::Result<()> {
    println!( "Enter the file name to delete:");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();
    match fs::remove_file(file_name) {
        Ok(_) => {
            println!("File '{}' deleted successfully.", file_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error deleting file '{}': {}", file_name, e);
            Err(e)
        }
    }
    Ok(())
    
}