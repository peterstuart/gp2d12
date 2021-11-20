use adc_interpolator::{AdcInterpolator, Config};
use core::{convert::TryFrom, fmt};
use embedded_hal::adc::{Channel, OneShot};

/// Driver for the GP2D12 infrared distance sensor.
#[derive(Debug)]
pub struct Gp2d12<Pin, Word> {
    interpolator: AdcInterpolator<Pin, Word, 18>,
}

type Error<Adc, ADC, Word, Pin> = nb::Error<<Adc as OneShot<ADC, Word, Pin>>::Error>;

/// # Examples
///
/// ```
/// use gp2d12::Gp2d12;
/// # use embedded_hal_mock::{
/// #     adc::{Mock, MockChan0, Transaction},
/// #     common::Generic,
/// #     MockError,
/// # };
/// #
/// # let expectations: [Transaction<u16>; 1] = [Transaction::read(0, 950)];
/// # let mut adc = Mock::new(&expectations);
/// # let pin = MockChan0 {};
///
/// // 3300 mV max voltage on the ADC, 12-bit precision
/// let mut gp2d12 = Gp2d12::new(pin, 3300, 12);
///
/// // measuring 40 cm
/// assert_eq!(gp2d12.distance(&mut adc), Ok(Some(40)));
/// ```
impl<Pin, Word> Gp2d12<Pin, Word> {
    /// Returns a `Gp2d12`.
    ///
    /// - `pin`: A pin configured as an analog input. The ADC associated with the pin must be used when calling [`distance`][Gp2d12::distance].
    /// - `max_voltage`: The voltage corresponding to the largest value possible for the ADC (mV)
    /// - `precision`: The precision of the ADC in bits (eg. for 10-bit precision, use `10`)
    pub fn new<ADC>(pin: Pin, max_voltage: u32, precision: u32) -> Self
    where
        Word: Copy + PartialOrd + TryFrom<u32>,
        <Word as TryFrom<u32>>::Error: fmt::Debug,
        Pin: Channel<ADC>,
    {
        let config = Config {
            max_voltage,
            precision,
            voltage_to_values: [
                (420, 80),
                (450, 75),
                (480, 70),
                (510, 65),
                (540, 60),
                (580, 55),
                (620, 50),
                (680, 45),
                (760, 40),
                (850, 35),
                (975, 30),
                (1140, 28),
                (1380, 20),
                (1520, 18),
                (1660, 16),
                (1860, 14),
                (2125, 12),
                (2450, 10),
            ],
        };
        let interpolator = AdcInterpolator::new(pin, config);

        Self { interpolator }
    }

    /// Destroys `self` and returns the `Pin`.
    pub fn free(self) -> Pin {
        self.interpolator.free()
    }

    /// Returns the distance measured by the sensor, in centimeters,
    /// if a distance can be determined.
    ///
    /// Note that distances of less than 10cm will produce incorrect
    /// values.
    pub fn distance<Adc, ADC>(
        &mut self,
        adc: &mut Adc,
    ) -> Result<Option<u32>, Error<Adc, ADC, Word, Pin>>
    where
        Word: Copy + Into<u32> + PartialEq + PartialOrd,
        Pin: Channel<ADC>,
        Adc: OneShot<ADC, Word, Pin>,
    {
        self.interpolator.read(adc)
    }

    /// Returns the minimum distance, in centimeters, that can be
    /// measured by the sensor.
    pub fn min_distance(&self) -> u32 {
        self.interpolator.min_value()
    }

    /// Returns the maximum distance, in centimeters, that can be
    /// measured by the sensor.
    pub fn max_distance(&self) -> u32 {
        self.interpolator.max_value()
    }
}
