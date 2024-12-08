use bl602_hal::gpio::I2c as gpio_i2c;
use bl602_hal::gpio::{Pin4, Pin3};
use bl602_hal::i2c::{Error, I2c as hal_i2c};
use bl602_pac::I2C as pac_i2c;
use embedded_hal::i2c::{I2c as embedded_i2c, SevenBitAddress};

/// The header size shared by 
/// all packets sent over SH-2.
const PACKET_HEADER_SIZE: usize = 4;

/// Max number of bytes allowed in the data
/// segment of a SH-2 packet. Limited by RAM.
const MAX_PACKET_SIZE: usize = 128;

const CHANNEL_COMMAND: u8 = 0;
const CHANNEL_EXECUTABLE: u8 = 1;
const CHANNEL_CONTROL: u8 = 2;
const CHANNEL_REPORT: u8 = 3;
const CHANNEL_WAKE_REPORTS: u8 = 4;
const CHANNEL_GYRO: u8 = 5;

/// Packet structure for data sent to 
/// SH-2.
struct Packet {
	header: [u8; PACKET_HEADER_SIZE],
	data: [u8; MAX_PACKET_SIZE],
}

/// A report structure that all Sh2 features use.
/// Sensitivity may be absolute or relative, depends
/// on the feature being set/reported.
/// 
/// This is metadata about the feature, not the input
/// report.
pub struct FeatureReport {
	feature_flag: u8,
	sensitivity: u16,
	report_interval: u32,
	batch_interval: u32,
	configuration_word: u32
}

pub struct Sh2<'a>{
	i2c: &'a mut hal_i2c<pac_i2c, (Pin4<gpio_i2c>, Pin3<gpio_i2c>)>,
}

impl<'a> Sh2<'a>{
	
	pub fn new(i2c: &'a mut hal_i2c<pac_i2c, (Pin4<gpio_i2c>, Pin3<gpio_i2c>)>) -> Sh2<'a>{
		Self{
			i2c: i2c
		}
	}

	/// Tells the BNO08x to begin reporting values for the specific report ID.
	pub fn set_feature(feature_id: u8, 
		interval: u16, 
		sensitivity: u16,
		configuration: u32) -> Result<(), Error> {

		Ok(())
	}

	/// Tells the BN#O08x to begin reporting values for the FeatureReport.
	pub fn set_feature_with_report(feature_report: FeatureReport) -> Result<(), Error> {
		Ok(())
	}

	/// Gets a FeatureReport for the feature_id. 
	pub fn get_feature(feature_id: u8) -> Result<FeatureReport, Error> {
		Ok(FeatureReport { feature_flag: 0, 
			sensitivity: 0, 
			report_interval: 0, 
			batch_interval: 0, 
			configuration_word: 0 })
	}

	fn send_packet(&mut self, address: SevenBitAddress, packet: Packet) -> Result<(), Error>{
		
		// Can't use vec from the std lib. Using packet.header and packet.data
		// as separate properties in the struct means we need to combine them to 
		// send it as a single i2c.write call.

		// Alternative is to send two separate i2c.write calls. Will improve this,
		// but will keep this to test at the moment.
		
		let mut message = [0u8; MAX_PACKET_SIZE + PACKET_HEADER_SIZE];
		message[..packet.header.len()].copy_from_slice(&packet.header);
		message[packet.header.len()..].copy_from_slice(&packet.data);

		return self.i2c.write(address, &message);
	}
}