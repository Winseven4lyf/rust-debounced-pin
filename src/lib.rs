//! Adds a wrapper for an `InputPin` that debounces it's `is_high()` and `is_low()` methods.

#![no_std]

use core::marker::PhantomData;
use embedded_hal::digital::v2::InputPin;

/// Unit struct for active-low pins.
pub struct ActiveLow;

/// Unit struct for active-high pins.
pub struct ActiveHigh;

/// The debounce state of the `update()` method
#[derive(PartialEq, Clone, Copy)]
pub enum DebounceState {
    /// The pin state is active, but not debounced
    Debouncing,
    /// The counter was reset
    Reset,
    /// The pin state is not active
    NotActive,
    /// The pin state is active and debounced
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
}

impl<T: InputPin> DebouncedInputPin<T, ActiveHigh> {
    /// Updates the debounce logic.
    ///
    /// Needs to be called every ~1ms.
    pub fn update(&mut self) -> Result<DebounceState, <Self as InputPin>::Error> {
        if self.pin.is_low()? {
            if self.debounce_state == DebounceState::Active {
                self.counter = 0;
                self.debounce_state = DebounceState::Reset;
            } else {
                self.debounce_state = DebounceState::NotActive;
            }
        } else if self.counter < 10 {
            self.counter += 1;
            self.debounce_state = DebounceState::Debouncing;
        } else {
            // Max count is reached
            self.debounce_state = DebounceState::Active;
        }

        Ok(self.debounce_state)
    }
}

impl<T: InputPin> DebouncedInputPin<T, ActiveLow> {
    /// Updates the debounce logic.
    ///
    /// Needs to be called every ~1ms.
    pub fn update(&mut self) -> Result<DebounceState, <Self as InputPin>::Error> {
        if self.pin.is_high()? {
            if self.debounce_state == DebounceState::Active {
                self.counter = 0;
                self.debounce_state = DebounceState::Reset;
            } else {
                self.debounce_state = DebounceState::NotActive;
            }
        } else if self.counter < 10 {
            self.counter += 1;
            self.debounce_state = DebounceState::Debouncing;
        } else {
            // Max count is reached
            self.debounce_state = DebounceState::Active;
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
