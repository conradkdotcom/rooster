#![macro_use]
#[allow(unused_imports)]
use std::io::Write;

#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        if let Err(x) = writeln!(&mut ::std::io::stderr(), $($arg)*) {
            panic!("Unable to write to stderr: {}", x);
        }
    )
);

#[macro_export]
macro_rules! println_err(
    ($($args:tt)*) => (
        println_stderr!(
            "{}",
            format!(
                "{}{}{}",
                ::color::Color::Red.to_color_code(),
                format!($($args)*),
                ::color::Color::Reset.to_color_code()
            )
        )
    )
);

#[macro_export]
macro_rules! println_ok(
    ($($args:tt)*) => (
        println_stderr!(
            "{}",
            format!(
                "{}{}{}",
                ::color::Color::Green.to_color_code(),
                format!($($args)*),
                ::color::Color::Reset.to_color_code()
            )
        )
    )
);

#[macro_export]
macro_rules! println_title(
    ($($args:tt)*) => (
        println_stderr!(
            "{}",
            format!(
                "{}{}{}",
                ::color::Color::Cyan.to_color_code(),
                format!($($args)*),
                ::color::Color::Reset.to_color_code()
            )
        )
    )
);

#[macro_export]
macro_rules! print_stderr(
    ($($arg:tt)*) => (
        match write!(::std::io::stderr(), $($arg)*) {
            Ok(_) => {
                if let Err(x) = ::std::io::stderr().flush() {
                    panic!("Unable to write to stderr: {}", x);
                }
            },
            Err(x) => panic!("Unable to write to stderr: {}", x);
        }
    )
);

#[macro_export]
macro_rules! print_stdout(
    ($($arg:tt)*) => (
        match write!(::std::io::stdout(), $($arg)*) {
            Ok(_) => {
                if let Err(x) = ::std::io::stdout().flush() {
                    panic!("Unable to write to stdout: {}", x);
                }
            },
            Err(x) => panic!("Unable to write to stdout: {}", x),
        }
    )
);
