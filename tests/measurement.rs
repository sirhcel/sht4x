use sht4x::{Measurement, SensorData};

const DATA_ZERO_ZERO: SensorData = SensorData {
    temperature: 0,
    humidity: 0,
};

const DATA_MAX_MAX: SensorData = SensorData {
    temperature: u16::MAX,
    humidity: u16::MAX,
};

#[test]
fn from_min_data() {
    let min: Measurement = DATA_ZERO_ZERO.into();
    assert_eq!(min.temperature_celsius(), -45);
    assert_eq!(min.humidity_percent(), -6);
}

#[test]
fn from_max_data() {
    let max = Measurement::from(DATA_MAX_MAX);
    assert_eq!(max.temperature_celsius(), -45 + 175);
    assert_eq!(max.humidity_percent(), -6 + 125);
}

#[test]
fn min_data_millis() {
    let min_data: Measurement = DATA_ZERO_ZERO.into();

    assert_eq!(min_data.temperature_milli_celsius(), -45000);
    assert_eq!(min_data.humidity_milli_percent(), -6000);
}

#[test]
fn max_data_millis() {
    let max_data = Measurement::from(DATA_MAX_MAX);

    assert_eq!(max_data.temperature_milli_celsius(), -45000 + 175000);
    assert_eq!(max_data.humidity_milli_percent(), -6000 + 125000);
}
