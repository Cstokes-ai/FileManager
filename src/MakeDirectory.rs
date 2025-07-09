// this file is to make a directory in rust.
//it uses a specifc 2 letter key called "cd" to create a directory
use std::fs;
use std::io::{self, Write};



//create a function for the cd recognition key to create a directory

pub fn cd_key() -> io::Result<()> {
    println!("Enter the 'cd' key to create a directory:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    match input {
        "cd" =>{
            make_directory()?;
            Ok(())
        }
        _ => {
            println!("Invalid input. Please enter 'cd' to create a directory.");
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))
        }
    }

}


pub fn make_directory() -> io::Result<()> {
    println!("Enter the name of the directory to crease:");
    let mut directory_name = String::new();
    io::stdin().read_line(&mut directory_name)?;
    let directory_name = directory_name.trim();
    fs::create_dir(directory_name)?;
    println!("Directory '{}' created successfully.", directory_name);
    Ok(())

}