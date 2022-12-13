When unwrapping `Option`s and `Result`s it's oddly 
annoying to print a simple error message and exit.
This crate introduces the `ErrExit` trait which allows exactly that.

For example,
```rust
use cmd_error::ErrExit;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_exit("file path not passed");
    
    let file = std::fs::read_to_string(&path)
        .unwrap_exit(&format!("file {} not found", path));

    // ...
}
```

Why not use `expect`? 
Panic message aren't for users, they're for the programmers.

This crate also exports the function `print_err_and_exit`
which unsurprisingly prints an error message and exits.

Exits with an exit code of 1.
Errors are printed to stderr.
