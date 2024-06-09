# Sensirion SHT4x Driver for Embedded HAL

A platform agnostic device driver for the Sensirion [SHT4x temperature and
humidity sensor
family](https://sensirion.com/resource/datasheet/sht4x).
It is based on [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)
traits and works in `no_std` environments.

In theory, it supports all of the sensor family's devices but has only been
tested with the SHT40-AD1B so far.

[![Build Status](https://github.com/sirhcel/sht4x/actions/workflows/ci.yml/badge.svg)](https://github.com/sirhcel/sht4x)
[![crates.io](https://img.shields.io/crates/v/sht4x.svg)](https://crates.io/crates/sht4x)
![Documentation](https://docs.rs/sht4x/badge.svg)


## Features

- Blocking operation
- Supports all commands specified in the
  [datasheet](https://sensirion.com/resource/datasheet/sht4x)
- Explicitly borrows `DelayMs` for command execution so that it could be shared
  (among multiple sensors)
- Could be instantiated with the alternative I2C address for the SHT40-BD1B
- Uses fixed-point arithmetics for converting raw sensor data into measurements
  in SI units
    - Based on `I16F16` from the [`fixed`](https://gitlab.com/tspiteri/fixed)
      crate
    - Allows conversion to floating-point values, if needed
    - Convenience methods for fixed-point conversions to milli degree Celsius
      or milli percent relative humidity which are commonly used by drivers for
      other humidity and temperature sensors from Sensirion
- Optional support for [`defmt`](https://github.com/knurling-rs/defmt)


## Example

```rust ignore
use sht4x::Sht4x;
// Device-specific use declarations.

let mut delay = // Device-specific initialization of delay.
let i2c = // Device-specific initialization of I2C peripheral.
let mut sht40 = Sht4x::new(i2c);

let serial = sht40.serial_number(&mut delay);
defmt::info!("serial number: {}", serial);

let measurement = sht40.measure(Precision::Low, &mut delay);
defmt::info!("measurement: {}", &measurement);

if let Ok(measurement) = measurement {
    // Convert temperature measurand into different formats for further
    // processing.
    let int: i32 = measurement.temperature_milli_celsius();
    let fixed: I16F16 = measurement.temperature_celsius();
    let float: f32 = measurement.temperature_celsius().to_num();
}
```

## `embedded-hal-async` support

This crate has optional support for the [`embedded-hal-async`] crate, which
provides `async` versions of the `I2c` and `DelayNs` traits. Async support
is an off-by-default optional feature, so that projects which aren't using
[`embedded-hal-async`] can avoid the additional dependency.

To use this crate with `embedded-hal-async`, enable the `embedded-hal-async`
feature flag in your `Cargo.toml`:

```toml
sht4x = { version = "0.2", features = ["embedded-hal-async"] }
```

Once the `embedded-hal-async` feature is enabled, construct an instance of
the `AsyncSht4x` struct, providing types implementing the
[`embedded_hal_async::i2c::I2c`] and [`embedded_hal_async::delay::DelayNs`]
traits. The `AsyncSht4x` struct is identical to the `Sht4x` struct,
except that its methods are `async fn`s.

[`embedded-hal-async`]: https://crates.io/crates/embedded-hal-async
[`embedded_hal_async::i2c::I2c`]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/i2c/trait.I2c.html
[`embedded_hal_async::delay::DelayNs`]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/delay/trait.DelayNs.html


## Related Work

[sensor-temp-humidity-sht40](https://github.com/lc525/sensor-temp-humidity-sht40-rs)
is another driver for this sensor family.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your discretion.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
