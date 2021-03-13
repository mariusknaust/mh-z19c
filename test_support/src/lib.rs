use crate::serial_mock::SerialMock;

pub mod serial_mock;

pub static READ_CO2_RESPONSE: [u8; 9] = [0xff, 0x86, 0x03, 0x20, 0x12, 0x34, 0x56, 0x78, 0x43];
pub static SELF_CALIBRATE_ON_COMMAND: [u8; 9] =
    [0xff, 0x01, 0x79, 0xa0, 0x00, 0x00, 0x00, 0x00, 0xe6];

pub fn create_serial_mock_returning(read_data: &[u8]) -> SerialMock {
    SerialMock::new(read_data.iter().copied().map(Ok).collect(), vec![Ok(()); 9])
}
