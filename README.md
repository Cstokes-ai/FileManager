# FileManagementRust

A simple command-line file management tool written in Rust. This project allows users to perform basic file and directory operations such as creating, reading, writing, deleting, and moving files and directories through a menu-driven interface.

## Features
- **Create Directory**: Make new directories by entering a special key ("cd") and specifying the directory name.
- **Write to File**: Create or overwrite files with custom content.
- **Read from File**: Display the contents of a specified file.
- **Delete File**: Remove files from the filesystem.
- **(Planned) Move/Copy Files**: Placeholder modules for moving and copying files.
- **(Planned) List/Remove Directory**: Placeholder modules for listing and removing directories.

## Usage
1. **Build the project:**
   ```sh
   cargo build
   ```
2. **Run the project:**
   ```sh
   cargo run
   ```
3. **Follow the menu prompts** to select an operation (e.g., create directory, write to file, etc.).

## Project Structure
- `src/LaunchMain.rs` - Main entry point and menu logic.
- `src/Filewrite.rs` - Handles writing to files.
- `src/FileRead.rs` - Handles reading from files.
- `src/FileDelete.rs` - Handles deleting files.
- `src/MakeDirectory.rs` - Handles creating directories with a special key.
- `src/FileMove.rs`, `src/FileCopy.rs`, `src/ListDirectory`, `src/RemoveDirectory.rs` - Placeholders for future features.

## Requirements
- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)

## Notes
- All file and directory operations are performed in the current working directory.
- Error handling is basic; invalid input or missing files/directories will print error messages.
- This project is for learning and demonstration purposes.

## License
MIT License
