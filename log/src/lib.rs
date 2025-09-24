extern crate log_base;

pub use log_base::*;

pub mod unfiltered_log {
    pub use log_base::*;
}

#[cfg(feature = "stealthy")]
#[doc(hidden)]
pub mod log {
    #[macro_export]
    macro_rules! error {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! warn {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! debug {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! info {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! trace {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! log {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! log_enabled {
        ($($tt:tt)*) => {
            false
        };
    }
}
