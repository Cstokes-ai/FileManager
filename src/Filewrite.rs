// This is for writing to a file in Rust

use std::io::{self, Write};
use std::path::Path;
mod fileRead;

use std::fs::File;
use std::io::Write;

pub fn searchforFile() {
    fileReading::searchforFile();

}
pub fn write_to_file() -> io::Result<()> {
    println!("Enter the file name to write to");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();
    let mut contents = String::new();
    println!("Enter the contents to write to the file (type 'exit' to finish):");
    
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let line = line.trim();
        
        if line.eq_ignore_ascii_case("exit") {
            break;
        }
        
        contents.push_str(line);
        contents.push('\n');
    }
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}


