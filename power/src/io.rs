use crate::*;

pub enum Output {
    Debug,
}

impl fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self {
            Output::Debug => debug(s),
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut x = Output::Debug;
        x.write_fmt(format_args!($($arg)*)).unwrap();
        flush();
    });
}

//use core::intrinsics;
//use core::panic::PanicInfo;

//#[panic_handler]
//fn panic(_info: &PanicInfo) -> ! {
//print!("PANIC!");
//unsafe { intrinsics::abort() }
//}

//#[alloc_error_handler]
//fn foo(_: core::alloc::Layout) -> ! {
//print!("Alloc Error!");
//unsafe { intrinsics::abort() }
//}
