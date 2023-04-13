mod folder_manager;

fn main() {
    match folder_manager::folder_reader::read_implementations_folder() {
        Ok(_) => println!("Reading implementations folder completed."),
        Err(e) => eprintln!("Error reading implementations folder: {}", e),
    }
}
