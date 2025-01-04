//! Internal timing functions and macros.

#[allow(clippy::module_inception)]
mod timers;
pub use timers::*;

#[cfg(feature = "zkEVM")]
mod timeless;
