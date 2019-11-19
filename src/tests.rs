use crate::Debounce;
use crate::DebounceState;
use crate::DebouncedInputPin;
use common::*;
use embedded_hal::digital::v2::InputPin;
use failure::Fail;
use mocks::*;

/// Mock implementations.
mod mocks {
    use super::*;

    #[derive(Debug, Fail)]
    #[fail(display = "An error occurred")]
    pub struct MockInputPinError;

    /// A mock implementation of `InputPin`.
    #[derive(Default)]
    pub struct MockInputPin {
        /// The state of the pin.
        pub state: bool,
    }

    impl InputPin for MockInputPin {
        type Error = MockInputPinError;

        fn is_high(&self) -> Result<bool, MockInputPinError> {
            Ok(self.state)
        }

        fn is_low(&self) -> Result<bool, MockInputPinError> {
            Ok(!self.state)
        }
    }
}

/// Shared functionallity for tests.
mod common {
    use super::*;

    /// Creates a `DebouncedInputPin<MockInputPin, A>`.
    pub fn create_pin<A>(activeness: A) -> DebouncedInputPin<MockInputPin, A> {
        let pin = MockInputPin::default();
        DebouncedInputPin::new(pin, activeness)
    }
}

/// Tests for `DebouncedInputPin<T, A>`.
mod input_pin {
    use super::*;

    /// Tests for `DebouncedInputPin<T, ActiveHigh>`.
    mod active_high {
        use super::*;
        use crate::ActiveHigh; // Not importing `ActiveHigh` further up the chain to prevent mistakes.

        #[test]
        fn it_updates_the_counter() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveHigh);
            pin.pin.state = true;
            assert_eq!(pin.counter, 0);
            pin.update()?;
            assert_eq!(pin.counter, 1);
            Ok(())
        }

        #[test]
        fn it_goes_active_when_counter_full() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveHigh);
            pin.pin.state = true;
            pin.counter = 10;
            assert!(pin.is_low()?);
            pin.update()?;
            assert_eq!(pin.counter, 10);
            assert!(pin.is_high()?);
            Ok(())
        }

        #[test]
        fn it_resets_the_counter_and_state_on_low() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveHigh);
            pin.pin.state = false;
            pin.counter = 10;
            pin.debounce_state = DebounceState::Active;
            assert!(pin.is_high()?);
            pin.update()?;
            assert!(pin.is_low()?);
            assert_eq!(pin.counter, 0);
            Ok(())
        }

        #[test]
        fn it_is_active_when_its_pin_state_is_high_and_vice_versa() -> Result<(), MockInputPinError>
        {
            let mut pin = create_pin(ActiveHigh);
            pin.debounce_state = DebounceState::Active;
            assert_eq!(pin.is_high()?, true);
            assert_eq!(pin.is_low()?, false);
            pin.debounce_state = DebounceState::NotActive;
            assert_eq!(pin.is_high()?, false);
            assert_eq!(pin.is_low()?, true);
            pin.debounce_state = DebounceState::Debouncing;
            assert_eq!(pin.is_high()?, false);
            assert_eq!(pin.is_low()?, true);
            pin.debounce_state = DebounceState::Reset;
            assert_eq!(pin.is_high()?, false);
            assert_eq!(pin.is_low()?, true);
            Ok(())
        }

        #[test]
        fn it_returns_expected_state_when_calling_update() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveHigh);

            pin.pin.state = false;
            assert_eq!(pin.update()?, DebounceState::NotActive);
            pin.pin.state = true;
            assert_eq!(pin.update()?, DebounceState::Debouncing);
            pin.counter = 10;
            assert_eq!(pin.update()?, DebounceState::Active);
            pin.pin.state = false;
            assert_eq!(pin.update()?, DebounceState::Reset);
            assert_eq!(pin.update()?, DebounceState::NotActive);
            Ok(())
        }
    }

    /// Tests for `DebouncedInputPin<T, ActiveLow>`.
    mod active_low {
        use super::*;
        use crate::ActiveLow; // Not importing `ActiveLow` further up the chain to prevent mistakes.

        #[test]
        fn it_updates_the_counter() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveLow);
            pin.pin.state = false;
            assert_eq!(pin.counter, 0);
            pin.update()?;
            assert_eq!(pin.counter, 1);
            Ok(())
        }

        #[test]
        fn it_goes_active_when_counter_full() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveLow);
            pin.pin.state = false;
            pin.counter = 10;
            assert!(pin.is_high()?);
            pin.update()?;
            assert_eq!(pin.counter, 10);
            assert!(pin.is_low()?);
            Ok(())
        }

        #[test]
        fn it_resets_the_counter_and_state_on_high() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveLow);
            pin.pin.state = true;
            pin.counter = 10;
            pin.debounce_state = DebounceState::Active;
            assert!(pin.is_low()?);
            pin.update()?;
            assert!(pin.is_high()?);
            assert_eq!(pin.counter, 0);
            Ok(())
        }

        #[test]
        fn it_is_active_when_its_pin_state_is_low_and_vice_versa() -> Result<(), MockInputPinError>
        {
            let mut pin = create_pin(ActiveLow);
            pin.debounce_state = DebounceState::Active;
            assert_eq!(pin.is_high()?, false);
            assert_eq!(pin.is_low()?, true);
            pin.debounce_state = DebounceState::NotActive;
            assert_eq!(pin.is_high()?, true);
            assert_eq!(pin.is_low()?, false);
            pin.debounce_state = DebounceState::Debouncing;
            assert_eq!(pin.is_high()?, true);
            assert_eq!(pin.is_low()?, false);
            pin.debounce_state = DebounceState::Reset;
            assert_eq!(pin.is_high()?, true);
            assert_eq!(pin.is_low()?, false);
            Ok(())
        }

        #[test]
        fn it_returns_expected_state_when_calling_update() -> Result<(), MockInputPinError> {
            let mut pin = create_pin(ActiveLow);

            pin.pin.state = true;
            assert_eq!(pin.update()?, DebounceState::NotActive);
            pin.pin.state = false;
            assert_eq!(pin.update()?, DebounceState::Debouncing);
            pin.counter = 10;
            assert_eq!(pin.update()?, DebounceState::Active);
            pin.pin.state = true;
            assert_eq!(pin.update()?, DebounceState::Reset);
            assert_eq!(pin.update()?, DebounceState::NotActive);
            Ok(())
        }
    }
}
