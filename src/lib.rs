#![no_std]

//! An embedded-hal driver for the GP2D12 infrared distance sensor.
//!
//! Distance calibration is based on the values in the
//! [datasheet](https://media.digikey.com/pdf/Data%20Sheets/Sharp%20PDFs/GP2D12.pdf).
//!
//! # Examples
//!
//! ```
//! use gp2d12::Gp2d12;
//! # use embedded_hal_mock::{
//! #     adc::{Mock, MockChan0, Transaction},
//! #     common::Generic,
//! #     MockError,
//! # };
//! # let expectations: [Transaction<u32>; 1] = [Transaction::read(0, 950)];
//! # let mut adc = Mock::new(&expectations);
//! # let pin = MockChan0 {};
//!
//! // 3300 mV max voltage on the ADC, 12-bit precision
//! let mut gp2d12 = Gp2d12::new(pin, 3300, 12);
//!
//! // measuring 40 cm
//! assert_eq!(gp2d12.distance(&mut adc), Ok(Some(40)));
//! ```
//!
//! See the [STM32F3Discovery
//! example](https://github.com/peterstuart/gp2d12/blob/main/examples/stm32f3discovery/README.md)
//! for a complete example.

mod gp2d12;

pub use self::gp2d12::Gp2d12;
