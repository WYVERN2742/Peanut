const ARVR_STABILIZATION_GRV: u16 = 0x3E2E;

pub type SensorId = u8;

/// Input report produced by Game Rotation Vector 
pub struct Sh2GameRotationVector {
	/// Quarternion i component
	quat_i: u16, 

 	/// Quarternion j component
	quat_j: u16,

	/// Quarternion k component
	quat_k: u16,

	/// Quarternion real component
	quat_real: u16,
}

pub enum RecordType {
	GameRotation(Sh2GameRotationVector)
}

/// Structure of data received from 
/// Sh2 sensors.
pub struct Sh2SensorValue {

	/// ID of the sensor that produced this event.
	sensor_id: SensorId,

	/// Increments once for each report sent. 
	/// Gaps in the sequence number indicate missing
	/// or dropped reports.
	sequence: u8,

	/// Status of the sensor
	/// 0 - Unreliable
	/// 1 - Low Accuracy
	/// 2 - Medium Accuracy
	/// 3 - High Accuracy
	status: u8,

	/// [uS]
	timestamp: u64,

	/// [uS] value is delay * 2^exponent (see status)
	delay: u32,

	/// Data received from 
	data: RecordType,
}

pub struct Sh2 {
	
}

impl Sh2 {
	// pub fn new() -> Sh2 {
		
	// }
}