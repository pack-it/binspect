/// Helper macro to print error logging in an error style.
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}: {}", colored::Colorize::bold(colored::Colorize::red("ERROR")), format_args!($($arg)*))
    };
}

pub(crate) use error;
