// src/timeless.rs
#![allow(dead_code)] // to suppress warnings if you don't use everything

// This module is compiled only if feature = "zkEVM" is set.
#[cfg(feature = "zkEVM")]
pub mod zkevm {
    use core::ops::{Add, Sub, AddAssign};

    // Mock Duration
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
    pub struct Duration {
        pub micros: u64, // or whatever internal representation you like
    }

    impl Duration {
        pub const ZERO: Self = Self { micros: 0 };

        pub const fn from_micros(micros: u64) -> Self {
            Self { micros }
        }
        pub fn as_micros(&self) -> u64 {
            self.micros
        }
        pub fn zero() -> Self {
            Self { micros: 0 }
        }
        pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
            self.micros.checked_sub(rhs.micros).map(|m| Self { micros: m })
        }
        pub fn checked_add(&self, rhs: Self) -> Option<Self> {
            self.micros.checked_add(rhs.micros).map(|m| Self { micros: m })
        }

        pub fn as_secs_f64(&self) -> f64 {
            // If `micros` is your representation,
            // convert microseconds to seconds as an f64:
            self.micros as f64 / 1_000_000.0
        }
    }

    impl Add for Duration {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                micros: self.micros + rhs.micros,
            }
        }
    }

    impl Sub for Duration {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                micros: self.micros - rhs.micros,
            }
        }
    }

    impl AddAssign for Duration {
        fn add_assign(&mut self, rhs: Self) {
            self.micros += rhs.micros;
        }
    }

    // A static counter to emulate monotonic "time"
    static mut COUNTER: u64 = 0;

    // Mock Instant
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub struct Instant {
        pub micros: u64,
    }

    impl Instant {
        pub fn now() -> Instant {
            // In a real environment, you might read a cycle counter or some env variable.
            // For a trivial increment approach:
            unsafe {
                COUNTER += 1;
                Instant { micros: COUNTER }
            }
        }

        // Mimic standard `Instant::elapsed()`
        pub fn elapsed(&self) -> Duration {
            let now = Self::now();
            Duration {
                micros: now.micros.saturating_sub(self.micros),
            }
        }

        pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
            self.micros.checked_sub(other.micros).map(|m| Duration { micros: m })
        }

        pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
            self.micros
                .checked_add(other.micros)
                .map(|m| Instant { micros: m })
        }

        pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
            self.micros
                .checked_sub(other.micros)
                .map(|m| Instant { micros: m })
        }
    }
}