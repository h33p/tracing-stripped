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
    pub fn noop_span() -> tracing_base::Span {
        tracing_base::trace_span!("")
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
    pub use tracing_base::event::*;
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
            $crate::noop::noop_span()
        };
    }
    #[macro_export]
    macro_rules! info_span {
        ($($tt:tt)*) => {
            $crate::noop::noop_span()
        };
    }
    #[macro_export]
    macro_rules! debug_span {
        ($($tt:tt)*) => {
            $crate::noop::noop_span()
        };
    }
    #[macro_export]
    macro_rules! trace_span {
        ($($tt:tt)*) => {
            $crate::noop::noop_span()
        };
    }
    #[macro_export]
    macro_rules! warn_span {
        ($($tt:tt)*) => {
            $crate::noop::noop_span()
        };
    }
    pub use super::{debug_span, info_span, span, trace_span, warn_span};
    pub use tracing_base::span::*;
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
    pub use tracing_base::log::*;
}
