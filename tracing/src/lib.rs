extern crate tracing_base;

#[cfg(feature = "stealthy")]
extern crate noop_attr;

#[cfg(feature = "stealthy")]
pub use noop_attr::noop as instrument;
pub use tracing_base::*;

pub mod unfiltered_tracing {
    pub use tracing_base::*;
}

#[cfg(feature = "stealthy")]
pub mod noop {
    #[derive(Clone, Copy)]
    pub struct NoopSpan;

    impl NoopSpan {
        pub fn in_scope<T>(self, f: impl FnOnce() -> T) -> T {
            f()
        }
    }
}

#[cfg(feature = "stealthy")]
pub mod event {
    #[macro_export]
    macro_rules! event {
        ($($tt:tt)*) => {
            ()
        };
    }
    #[macro_export]
    macro_rules! event_enabled {
        ($($tt:tt)*) => {
            false
        };
    }
    pub use super::{event, event_enabled};
}

#[cfg(feature = "stealthy")]
pub mod span {
    #[macro_export]
    macro_rules! span_enabled {
        ($($tt:tt)*) => {
            false
        };
    }
    #[macro_export]
    macro_rules! span {
        ($($tt:tt)*) => {
            $crate::noop::NoopSpan
        };
    }
    #[macro_export]
    macro_rules! info_span {
        ($($tt:tt)*) => {
            $crate::noop::NoopSpan
        };
    }
    #[macro_export]
    macro_rules! debug_span {
        ($($tt:tt)*) => {
            $crate::noop::NoopSpan
        };
    }
    #[macro_export]
    macro_rules! trace_span {
        ($($tt:tt)*) => {
            $crate::noop::NoopSpan
        };
    }
    #[macro_export]
    macro_rules! warn_span {
        ($($tt:tt)*) => {
            $crate::noop::NoopSpan
        };
    }
    pub use super::{debug_span, info_span, span, trace_span, warn_span};
}

#[cfg(feature = "stealthy")]
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
    pub use super::{debug, error, info, trace, warn};
}
