use crate::types::SensorData;

const PAYLOAD_LEN: usize = 4;
pub(crate) const RESPONSE_LEN: usize = 6;

fn response_payload(response: [u8; RESPONSE_LEN]) -> [u8; PAYLOAD_LEN] {
    // Response data comes in chunks of three bytes: [MSB, LSB, CRC]. The CRCs got already checked
    // by sensirion_i2c::read_words_with_crc. So we just have to extract the payload data here.
    [response[0], response[1], response[3], response[4]]
}

pub(crate) fn sensor_data_from_response(response: [u8; RESPONSE_LEN]) -> SensorData {
    let payload = response_payload(response);
    SensorData {
        temperature: u16::from_be_bytes([payload[0], payload[1]]),
        humidity: u16::from_be_bytes([payload[2], payload[3]]),
    }
}

pub(crate) fn serial_number_from_response(response: [u8; RESPONSE_LEN]) -> u32 {
    let payload = response_payload(response);
    u32::from_be_bytes(payload)
}
