//! Hash Compiler timing and profiling utilities.

pub mod path;
pub mod printing;
pub mod testing;
pub mod tree_writing;

use log::log_enabled;
use std::time::{Duration, Instant};

#[macro_export]
macro_rules! counter {
    (
        name: $name:ident,
        counter_name: $counter_name:ident,
        visibility: $visibility:vis,
        method_visibility: $method_visibility:vis,
        derives: ($($derive:ident),*),
    ) => {
        static $counter_name: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

        #[derive($($derive),*)]
        $visibility struct $name(pub u32);

        impl $name {
            $method_visibility fn new() -> Self {
                Self($counter_name.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
            }

            $method_visibility fn total() -> u32 {
                $counter_name.load(std::sync::atomic::Ordering::SeqCst)
            }
        }

        impl From<u32> for $name {
            fn from(id: u32) -> Self {
                $name(id)
            }
        }

        impl From<$name> for u32 {
            fn from(counter: $name) -> u32 {
                counter.0
            }
        }
    };
    (
        name: $name:ident,
        counter_name: $counter_name:ident,
        visibility: $visibility:vis,
        method_visibility: $method_visibility:vis,
    ) => {
        counter! {
            name: $name,
            counter_name: $counter_name,
            visibility: $visibility,
            method_visibility: $method_visibility,
            derives: (Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd),
        }
    };
}

#[inline(always)]
pub fn timed<T>(op: impl FnOnce() -> T, level: log::Level, on_elapsed: impl FnOnce(Duration)) -> T {
    if log_enabled!(level) {
        let begin = Instant::now();
        let result = op();
        on_elapsed(begin.elapsed());
        result
    } else {
        op()
    }
}
