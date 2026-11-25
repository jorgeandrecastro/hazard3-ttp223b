// Copyright (C) 2026 Jorge Andre Castro
// GPL-3.0-or-later

#![no_std]
#![forbid(unsafe_code)]

//! # hazard3-ttp223b
//!
//! Asynchronous `no_std` GPIO driver for the TTP223B touch sensor.
//! Filters false positives with software debouncing without blocking waits.
//!
//! Part of the independent
//! [Hazard3 Async](https://github.com/jorgeandrecastro/hazard3-async).
//! Works with a pin implementing `embedded_hal::digital::InputPin`.
//! The default backend uses `hazard3-time`; for upstream Embassy, disable
//! default features and enable `time-upstream`.
//!
//! # Example
//!
//! ```rust,no_run
//! use embedded_hal::digital::InputPin;
//! use hazard3_ttp223b::{Ttp223, TouchState};
//!
//! async fn watch<P: InputPin>(pin: P) {
//!     let mut touch = Ttp223::new(pin);
//!
//!     loop {
//!         match touch.wait_for_change().await {
//!             Ok(TouchState::Pressed)  => { /* handle touch */ }
//!             Ok(TouchState::Released) => { /* handle release */ }
//!             Err(_) => { /* handle error */ }
//!         }
//!     }
//! }
//! ```

#[cfg(all(feature = "time-hazard3", feature = "time-upstream"))]
compile_error!("Features \"time-hazard3\" and \"time-upstream\" are mutually exclusive.");

#[cfg(not(any(feature = "time-hazard3", feature = "time-upstream")))]
compile_error!("Select one time backend: \"time-hazard3\" or \"time-upstream\".");

#[cfg(feature = "time-upstream")]
extern crate embassy_time;

#[cfg(all(feature = "time-hazard3", not(feature = "time-upstream")))]
extern crate time_hazard3_backend as embassy_time;

use embassy_time::{Duration, Timer};
use embedded_hal::digital::InputPin;

/// Default debounce duration (ms).
pub const DEBOUNCE_MS: u64 = 50;

/// Default polling interval when no change is detected (ms).
pub const POLL_MS: u64 = 10;

/// Possible touch sensor states.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TouchState {
    /// The sensor is pressed (high signal).
    Pressed,
    /// The sensor is released (low signal).
    Released,
}

/// TTP223B sensor errors.
#[derive(Debug)]
pub enum TouchError {
    /// GPIO hardware read error.
    ReadError,
    /// The signal oscillates too much to be considered reliable.
    SignalUnstable,
}

/// Main TTP223B driver instance.
///
/// Works with any pin implementing `embedded_hal::digital::InputPin`.
pub struct Ttp223<P: InputPin> {
    pin: P,
    last_confirmed_state: bool,
    debounce_ms: u64,
    poll_ms: u64,
}

impl<P: InputPin> Ttp223<P> {
    /// Initializes a new driver instance with a GPIO pin.
    ///
    /// Uses the default debounce and polling durations
    /// ([`DEBOUNCE_MS`] and [`POLL_MS`]).
    ///
    /// # Arguments
    /// * `pin` — GPIO pin configured as an input.
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            last_confirmed_state: false,
            debounce_ms: DEBOUNCE_MS,
            poll_ms: POLL_MS,
        }
    }

    /// Initializes a new instance with custom timings.
    ///
    /// # Arguments
    /// * `pin` — GPIO pin configured as an input.
    /// * `debounce_ms` — Stabilization wait duration (ms).
    /// * `poll_ms` — Polling interval when there is no change (ms).
    pub fn new_with_timing(pin: P, debounce_ms: u64, poll_ms: u64) -> Self {
        Self {
            pin,
            last_confirmed_state: false,
            debounce_ms,
            poll_ms,
        }
    }

    /// Waits for a state change with timed confirmation (debounce).
    ///
    /// This asynchronous function yields to the executor during waits
    /// ([`Timer`]) without blocking the CPU.
    ///
    /// # Returns
    /// * `Ok(TouchState::Pressed)` — Confirmed touch.
    /// * `Ok(TouchState::Released)` — Confirmed release.
    /// * `Err(TouchError::ReadError)` — GPIO read error.
    pub async fn wait_for_change(&mut self) -> Result<TouchState, TouchError> {
        loop {
            let current_raw = self.pin.is_high().map_err(|_| TouchError::ReadError)?;

            if current_raw != self.last_confirmed_state {
                // Debounce: wait for electrical bouncing to settle.
                Timer::after(Duration::from_millis(self.debounce_ms)).await;

                let confirmed = self.pin.is_high().map_err(|_| TouchError::ReadError)?;

                if confirmed == current_raw {
                    self.last_confirmed_state = confirmed;
                    return Ok(if confirmed {
                        TouchState::Pressed
                    } else {
                        TouchState::Released
                    });
                }
                // Ignore an unstable signal and retry.
            }

            // Brief pause to avoid overwhelming the scheduler.
            Timer::after(Duration::from_millis(self.poll_ms)).await;
        }
    }

    /// Returns the stored state without waiting for another change.
    ///
    /// Useful for checking the current state in a periodic task.
    pub fn get_state(&self) -> TouchState {
        if self.last_confirmed_state {
            TouchState::Pressed
        } else {
            TouchState::Released
        }
    }

    /// Indicates whether the sensor is pressed (stored state).
    pub fn is_pressed(&self) -> bool {
        self.last_confirmed_state
    }

    /// Releases the GPIO pin and returns the hardware resource.
    pub fn release(self) -> P {
        self.pin
    }
}
