/// Variable argument version of `syscall`
#[macro_export]
macro_rules! syscall {
    ($nr:ident, $a1:expr, $a2:expr) => {
        $crate::syscall($crate::nr::$nr, &($a1 as usize))
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize,
                                           $a3 as usize])
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall($crate::nr::$nr, &[$a1 as usize, $a2 as usize,
                                           $a3 as usize, $a4 as usize])
    };
}
/// Macro for printing to the **host's** standard output
#[macro_export]
macro_rules! hprint {
    ($s:expr) => ($crate::io::write_str($s));
    ($($arg:tt)*) => ($crate::io::write_fmt(format_args!($($arg)*)));
}

/// Macro for printing to the **host's** standard output, with a newline.
#[macro_export]
macro_rules! hprintln {
    () => (print!("\n"));
    ($fmt:expr) => (hprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (hprint!(concat!($fmt, "\n"), $($arg)*));
}
