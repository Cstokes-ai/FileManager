mod Filewrite;
mod FileRead;
mod FileDelete;
mod make_directory;


fn main() {

    //choice for decision
    println!("Enter a choice (1-8):");
    println!("1. Make Directory");
    println!("2. Write to File");
    println!("3. Read from File");
    println!("4. Delete File");
    println!("5. Exit");
    
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().parse::<u32>().expect("Invalid input");

    match choice {
        1 => {
            if let Err(e) = make_directory::cd_key() {
                eprintln!("Error creating directory: {}", e);
            }
        }
        2 => {
            if let Err(e) = Filewrite::write_to_file() {
                eprintln!("Error writing to file: {}", e);
            }
        }
        3 => {
            if let Err(e) = FileRead::read_from_file() {
                eprintln!("Error reading from file: {}", e);
            }
        }
        4 => {
            if let Err(e) = FileDelete::delete_file() {
                eprintln!("Error deleting file: {}", e);
            }
        }
        5 => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            eprintln!("Invalid choice");
        }
    }

}