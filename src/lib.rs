//! A platform-agnostic debounce library.
//!
//! This library provides an `update()` method to debounce a pin.
//!
//! Implements approach 1 from [here](http://www.labbookpages.co.uk/electronics/debounce.html#soft)
//! ([archived 2018-09-03](https://web.archive.org/web/20180903142143/http://www.labbookpages.co.uk/electronics/debounce.html#soft)).
//!
//! It also adds a wrapper for an `InputPin` that debounces it's
//! `is_high()` and `is_low()` methods.
//!
//! # Implementation
//!
//! The `InputPin` wrapper checks **only** the debounced state.
//! It does not poll the pin and drives the debouncing poll implementation forward.
//! To do this, you have to call `update()`. At best call it every 1ms in an ISR.
//!
//! # Example
//!
//! For examples check the [examples](https://github.com/Winseven4lyf/rust-debounced-pin/tree/master/examples)
//! directory in the repository.
//!
//! ```rust,ignore
//! use debounced_pin::prelude::*;
//! use debounced_pin::ActiveHigh;
//!
//! // This is up to the implementation details of the embedded_hal you are using.
//! let pin: InputPin = hal_function_which_returns_input_pin();
//!
//! let pin = DebouncedInputPin::new(pin, ActiveHigh);
//!
//! loop {
//!     match pin.update()? {
//!         // Pin is not active.
//!         DebounceState::NotActive => break,
//!         // Pin was reset or is not active in general.
//!         DebounceState::Reset => break,
//!         // Pin is active but still debouncing.
//!         DebounceState::Debouncing => continue,
//!         // Pin is active and debounced.
//!         DebounceState::Active => break,
//!     }
//!     // Wait to poll again in 1ms. Also hardware specific.
//!     wait(1.ms());
//! }
//!
//! // If the debounce state is DebounceState::Active
//! // this returns true and the code gets executed,
//! // else this false.
//! if pin.is_high()? {
//!     // Do something with it
//!     break;
//! }
//! ```

#![no_std]

pub mod prelude;

use core::marker::PhantomData;
use embedded_hal::digital::v2::InputPin;

/// Unit struct for active-low pins.
pub struct ActiveLow;

/// Unit struct for active-high pins.
pub struct ActiveHigh;

/// The debounce state of the `update()` method.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DebounceState {
    /// The pin state is active, but not debounced.
    Debouncing,
    /// The counter was reset.
    Reset,
    /// The pin state is not active.
    NotActive,
    /// The pin state is active and debounced.
    Active,
}

/// A debounced input pin.
///
/// Implements approach 1 from [here](http://www.labbookpages.co.uk/electronics/debounce.html#soft)
/// ([archived 2018-09-03](https://web.archive.org/web/20180903142143/http://www.labbookpages.co.uk/electronics/debounce.html#soft)).
///
/// Requires `update()` to be called every ~1ms.
pub struct DebouncedInputPin<T: InputPin, A> {
    /// The wrapped pin.
    pub pin: T,

    /// Whether the pin is active-high or active-low.
    activeness: PhantomData<A>,

    /// The debounced pin state.
    debounce_state: DebounceState,

    /// The counter.
    counter: i8,
}

/// Debounce Trait which provides an `update()` method which debounces the pin.
pub trait Debounce {
    type Error;
    type State;

    fn update(&mut self) -> Result<Self::State, Self::Error>;
}

impl<T: InputPin, A> DebouncedInputPin<T, A> {
    /// Initializes a new debounced input pin.
    pub fn new(pin: T, _activeness: A) -> Self {
        Self {
            pin,
            activeness: PhantomData,
            counter: 0,
            debounce_state: DebounceState::NotActive,
        }
    }

    /// Checks if the pin debounce state is active.
    pub fn is_active(&self) -> bool {
        self.debounce_state == DebounceState::Active
    }
}

impl<T: InputPin> Debounce for DebouncedInputPin<T, ActiveHigh> {
    type Error = T::Error;
    type State = DebounceState;

    /// Updates the debounce logic.
    ///
    /// Needs to be called every ~1ms.
    fn update(&mut self) -> Result<Self::State, Self::Error> {
        if self.pin.is_low()? {
            if self.debounce_state == Self::State::Active {
                self.counter = 0;
                self.debounce_state = Self::State::Reset;
            } else {
                self.debounce_state = Self::State::NotActive;
            }
        } else if self.counter < 10 {
            self.counter += 1;
            self.debounce_state = Self::State::Debouncing;
        } else {
            // Max count is reached
            self.debounce_state = Self::State::Active;
        }

        Ok(self.debounce_state)
    }
}

impl<T: InputPin> Debounce for DebouncedInputPin<T, ActiveLow> {
    type Error = T::Error;
    type State = DebounceState;

    /// Updates the debounce logic.
    ///
    /// Needs to be called every ~1ms.
    fn update(&mut self) -> Result<Self::State, Self::Error> {
        if self.pin.is_high()? {
            if self.debounce_state == Self::State::Active {
                self.counter = 0;
                self.debounce_state = Self::State::Reset;
            } else {
                self.debounce_state = Self::State::NotActive;
            }
        } else if self.counter < 10 {
            self.counter += 1;
            self.debounce_state = Self::State::Debouncing;
        } else {
            // Max count is reached
            self.debounce_state = Self::State::Active;
        }

        Ok(self.debounce_state)
    }
}

impl<T: InputPin> InputPin for DebouncedInputPin<T, ActiveHigh> {
    type Error = T::Error;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.debounce_state == DebounceState::Active)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.debounce_state != DebounceState::Active)
    }
}

impl<T: InputPin> InputPin for DebouncedInputPin<T, ActiveLow> {
    type Error = T::Error;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.debounce_state != DebounceState::Active)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.debounce_state == DebounceState::Active)
    }
}

#[cfg(test)]
mod tests;
