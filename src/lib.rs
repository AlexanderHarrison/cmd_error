pub trait ErrExit {
    type Inner;
    fn unwrap_exit(self, error_message: &str) -> Self::Inner;
}

pub fn print_err_and_exit(error_message: &str) -> ! {
    eprintln!("{}", error_message);
    std::process::exit(1);
}

impl<T> ErrExit for Option<T> {
    type Inner = T;
    fn unwrap_exit(self, error_message: &str) -> Self::Inner {
        match self {
            Some(n) => n,
            None => print_err_and_exit(error_message),
        }
    }   
}

impl<O, E> ErrExit for Result<O, E> {
    type Inner = O;
    fn unwrap_exit(self, error_message: &str) -> Self::Inner {
        match self {
            Ok(n) => n,
            Err(_) => print_err_and_exit(error_message),
        }
    }
}
