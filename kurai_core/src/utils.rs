use colored::Colorize;

#[macro_export]
macro_rules! print_error {
    ($($msg:tt)*) => {
        eprintln!("{}: {}", "error".red().bold(), format!($($msg)*))
    };
}

#[macro_export]
macro_rules! print_hint {
    ($($msg:tt)*) => {
        eprintln!("{}: {}", "hint".cyan().bold(), format!($($msg)*))
    };
}

#[macro_export]
macro_rules! kurai_panic {
    () => {
        #[cfg(debug_assertions)]
        panic!("Compilation terminated due to previous error");

        #[allow(unreachable_code)] {
            eprintln!("Compilation terminated due to previous error");
            std::process::exit(1);
        }
    };
}
