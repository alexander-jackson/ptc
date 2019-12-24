use std::error::Error;

pub fn handle_errors(result: Result<(), Box<dyn Error>>) {
    if result.is_ok() {
        return;
    }

    let error = result.err().unwrap();
    eprintln!("Error: {}", error);
}
