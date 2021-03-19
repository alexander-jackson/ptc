fn main() {
    let result = ptc::app::get_arguments().and_then(|args| ptc::app::process_args(&args));

    if let Err(e) = result {
        eprintln!("Error occurred: {}", e);
    }
}
