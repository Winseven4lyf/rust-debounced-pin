//! Adds a wrapper for an `InputPin` that debounces it's `is_high()` and `is_low()` methods.

#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use embedded_hal::digital::InputPin;

/// Unit struct for active-low pins.
pub struct ActiveLow;

/// Unit struct for active-high pins.
pub struct ActiveHigh;

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

    /// The counter.
    counter: i8,

    /// The debounced state.
    state: bool,
}

impl<T: InputPin, A> DebouncedInputPin<T, A> {
    /// Initializes a new debounced input pin.
    pub fn new(pin: T) -> Self {
        DebouncedInputPin {
            pin,
            activeness: PhantomData,
            counter: 0,
            state: false,
        }
    }
}

impl<T: InputPin, A> InputPin for DebouncedInputPin<T, A> {
    fn is_high(&self) -> bool {
        self.state
    }

    fn is_low(&self) -> bool {
        !self.state
    }
}

impl<T: InputPin> DebouncedInputPin<T, ActiveHigh> {
    /// Updates the debounce logic.
    ///
    /// Needs to be called every ~1ms.
    pub fn update(&mut self) {
        if self.pin.is_low() {
            self.counter = 0;
            self.state = false;
        } else if self.counter < 10 {
            self.counter += 1;
        }

        if self.counter == 10 {
            self.state = true;
        }
    }
}

impl<T: InputPin> DebouncedInputPin<T, ActiveLow> {
    /// Updates the debounce logic.
    ///
    /// Needs to be called every ~1ms.
    pub fn update(&mut self) {
        if self.pin.is_high() {
            self.counter = 0;
            self.state = false;
        } else if self.counter < 10 {
            self.counter += 1;
        }

        if self.counter == 10 {
            self.state = true;
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO implement tests
}
