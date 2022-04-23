# Sensirion SHT4x Driver for Embedded HAL

This is a device driver for the Sensirion [SHT4x temperature and humidity
sensor
family](https://sensirion.com/media/documents/33FD6951/624C4357/Datasheet_SHT4x.pdf)
based on [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)
traits.

In theory, it supports all of the sensor family's devices but has only been
tested with the SHT40-AD1B so far.

[![Build Status](https://github.com/sirhcel/sht4x/actions/workflows/ci.yml/badge.svg)](https://github.com/sirhcel/sht4x)
[![crates.io](https://img.shields.io/crates/v/sht4x.svg)](https://crates.io/crates/sht4x)
![Documentation](https://docs.rs/sht4x/badge.svg)


## Features

- Blocking operation
- Supports all commands specified in the
  [datasheet](https://sensirion.com/media/documents/33FD6951/624C4357/Datasheet_SHT4x.pdf)
- Explicitly borrows `DelayMs` for command execution so that it could be shared
  (among multiple sensors)
- Could be instatiated with the alternative I2C address for the SHT40-BD1B
- Uses fixed-point arithmetics for converting raw sensor data into measurements
  in SI unit
    - Based on `I16F16` from the [`fixed`](https://gitlab.com/tspiteri/fixed)
      crate
    - Allows conversion to floating-point values, if needed
    - Convenience methods for fixed-point conversions to milli degree Celsius
      or milli percent relative humidity which are commonly used by drivers for
      other humidity and temperature sensors from Sensirion
- Optional support for [`defmt`](https://github.com/knurling-rs/defmt)


## Example


## Related Work

[sensor-temp-humidity-sht40](https://github.com/lc525/sensor-temp-humidity-sht40-rs)
is another driver for this sensor family.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your discretion.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
