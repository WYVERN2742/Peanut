use embedded_hal::i2c::{I2c, SevenBitAddress};
use bl602_hal::i2c::Error;
use super::sh2::{SensorId, RecordType, Sh2SensorValue};

pub struct BNO085<'a> {
    address: SevenBitAddress,
    name: &'a str,
    initialized: bool,
}

impl<'a> BNO085<'a> {
	pub fn new(address: SevenBitAddress, name:&'a str) -> BNO085<'a> {
		Self {address: address, name: name, initialized: false}
	}

    // Initializes comms with the BNO085 module over I2C.
    pub fn init(&self) -> Result<(), Error>{
        Ok(())
    }

    // Resets the BNO085 hardware.
    pub fn reset() -> Result<(), Error>{
        Ok(())
    }

    /// Enables a report/feature to start reporting
    /// inputs from the sensor. 
    pub fn enable_report(sensor_id: SensorId, interval: u32) -> Result<(), Error> {
        Ok(())
    }

    /// Disables a report/feature to stop reporting
    /// inputs from the sensor.
    pub fn disable_report(sensor_id: SensorId) -> Result<(), Error>{
        Ok(())
    }

    /// Fills sensor value with report data, if data is available.
    /// Result will be false if there is no report available.
    pub fn get_sensor_event(sensor_value: &mut Sh2SensorValue) -> Result<bool, Error>{
        Ok(false)
    }
}
