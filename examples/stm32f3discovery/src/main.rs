#![no_main]
#![no_std]

use defmt_rtt as _;
use gp2d12::Gp2d12;
use panic_probe as _;
use stm32f3_discovery::stm32f3xx_hal::{self as _, adc, delay::Delay, pac, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = peripherals.RCC.constrain();

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = peripherals.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    let mut gpioe = peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);

    let mut adc3 = adc::Adc::adc3(
        peripherals.ADC3,
        &mut peripherals.ADC3_4,
        &mut reset_and_clock_control.ahb,
        adc::CkMode::default(),
        clocks,
    );

    let pin = gpioe.pe7.into_analog(&mut gpioe.moder, &mut gpioe.pupdr);
    let mut gp2d12 = Gp2d12::new(pin, 3300, 12);

    loop {
        let distance = gp2d12.distance(&mut adc3).unwrap();
        match distance {
            Some(distance) => defmt::println!("distance: {} cm", distance),
            None => defmt::println!("could not measure distance"),
        };

        delay.delay_ms(250u16);
    }
}
