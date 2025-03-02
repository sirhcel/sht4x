# Changelog

## [Unreleased]

### Added
### Changed
### Fixed
### Removed


## [0.2.0] - 2025-03-02

### Added

* Support for `embedded-hal-async` [#7](https://github.com/sirhcel/sht4x/pull/7)
* Implementations of common traits according to [Rust API Guidelines
  Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)
* New I2C address 0x46 for SHT40-CD1B-R3

### Changed

* embedded-hal from 0.2 to 1.0 [#3](https://github.com/sirhcel/sht4x/pull/4)
  and [#6](https://github.com/sirhcel/sht4x/pull/6)
* Made `Address` a non-exhaustive enum

### Fixed

* Missing local license files
* Link to datasheet


## [0.1.0] - 2022-12-01

* Initial release


[Unreleased]: https://github.com/sirhcel/sht4x/compare/0.2.0..HEAD
[0.2.0]: https://github.com/sirhcel/sht4x/releases/tag/0.2.0
[0.1.0]: https://github.com/sirhcel/sht4x/releases/tag/0.1.0
