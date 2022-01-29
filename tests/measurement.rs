use fixed::types::I16F16;
use sht4x::{Measurement, SensorData};

const DATA_ZERO_ZERO: SensorData = SensorData {
    temperature: 0,
    humidity: 0,
};

const DATA_MAX_MAX: SensorData = SensorData {
    temperature: u16::MAX,
    humidity: u16::MAX,
};

const MEASUREMENT_MIN_MIN: Measurement = Measurement {
    temperature_celsius: I16F16::MIN,
    humidity_percent: I16F16::MIN,
};

const MEASUREMENT_MAX_MAX: Measurement = Measurement {
    temperature_celsius: I16F16::MAX,
    humidity_percent: I16F16::MAX,
};

#[test]
fn from_min_data() {
    let min = Measurement::from(DATA_ZERO_ZERO);
    assert_eq!(min.temperature_celsius, -45);
    assert_eq!(min.humidity_percent, -6);
}

#[test]
fn from_max_data() {
    let max = Measurement::from(DATA_MAX_MAX);
    assert_eq!(max.temperature_celsius, -45 + 175);
    assert_eq!(max.humidity_percent, -6 + 125);
}

#[test]
#[should_panic]
fn min_temperature_milli_celsius() {
    assert!(MEASUREMENT_MIN_MIN.temperature_milli_celsius() < 0);
}

#[test]
#[should_panic]
fn min_humidity_milli_percent() {
    assert!(MEASUREMENT_MIN_MIN.humidity_milli_percent() < 0);
}

#[test]
fn min_data_millis() {
    let min_data = Measurement::from(DATA_ZERO_ZERO);

    assert_eq!(min_data.temperature_milli_celsius(), -45000);
    assert_eq!(min_data.humidity_milli_percent(), -6000);
}

#[test]
fn zero_millis() {
    let zeroes = Measurement {
        temperature_celsius: I16F16::ZERO,
        humidity_percent: I16F16::ZERO,
    };

    assert_eq!(zeroes.temperature_milli_celsius(), 0);
    assert_eq!(zeroes.humidity_milli_percent(), 0);
}

#[test]
fn max_data_millis() {
    let max_data = Measurement::from(DATA_MAX_MAX);

    assert_eq!(max_data.temperature_milli_celsius(), -45000 + 175000);
    assert_eq!(max_data.humidity_milli_percent(), -6000 + 125000);
}

#[test]
#[should_panic]
fn max_temperature_milli_celsius() {
    assert!(MEASUREMENT_MAX_MAX.temperature_milli_celsius() > 0);
}

#[test]
#[should_panic]
fn max_humidity_milli_percent() {
    assert!(MEASUREMENT_MAX_MAX.humidity_milli_percent() > 0);
}
