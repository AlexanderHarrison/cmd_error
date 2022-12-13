use cmd_error::ErrExit;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_exit("file path not passed");
    
    let file = std::fs::read_to_string(&path)
        .unwrap_exit(&format!("file {} not found", path));

    println!("{}", file);
}
