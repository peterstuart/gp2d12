# gp2d12 &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Badge]][docs] 
[Build Status]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fpeterstuart%2Fgp2d12%2Fbadge%3Fref%3Dmain&style=flat
[actions]: https://actions-badge.atrox.dev/peterstuart/gp2d12/goto?ref=main
[Latest Version]: https://img.shields.io/crates/v/gp2d12.svg
[crates.io]: https://crates.io/crates/gp2d12
[Docs Badge]: https://docs.rs/gp2d12/badge.svg
[docs]: https://docs.rs/gp2d12

An embedded-hal driver for the GP2D12 infrared distance sensor.

Distance calibration is based on the values in the
[datasheet](https://media.digikey.com/pdf/Data%20Sheets/Sharp%20PDFs/GP2D12.pdf).

## Examples

```rust
use gp2d12::Gp2d12;

// 3300 mV max voltage on the ADC, 12-bit precision
let mut gp2d12 = Gp2d12::new(pin, 3300, 12);

// measuring 40 cm
assert_eq!(gp2d12.distance(&mut adc), Ok(Some(40)));
```

See the [STM32F3Discovery
example](https://github.com/peterstuart/gp2d12/blob/main/examples/stm32f3discovery/README.md)
for a complete example.
