fn main() {
    let result = ptc::app::get_arguments().and_then(ptc::app::process_args);

    match result {
        Ok(_) => (),
        Err(e) => eprintln!("Error occurred: {}", e),
    };
}
