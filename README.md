# gp2d12

An embedded-hal driver for the GP2D12 infrared distance sensor.

Distance calibration is based on the values in the
[datasheet](https://media.digikey.com/pdf/Data%20Sheets/Sharp%20PDFs/GP2D12.pdf).

## Examples

```rust
use gp2d12::Gp2d12;

let mut gp2d12 = Gp2d12::new(pin);

// measuring 40 cm
assert_eq!(gp2d12.distance(&mut adc), Ok(Some(40)));
```

License: MIT
