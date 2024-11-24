const ARVR_STABILIZATION_GRV: u16 = 0x3E2E;

struct sh2 {
	
}

impl sh2 {
	pub fn new() -> sh2 {

	}

	pub fn device_reset(address: SevenBitAddress);
	pub fn device_on(address: SevenBitAddress);
	pub fn device_sleep(address: SevenBitAddress);
	pub fn device_open(address: SevenBitAddress);
	pub fn device_close(address: SevenBitAddress);
	pub fn device_service(address: SevenBitAddress);
	pub fn get_product_id(address: SevenBitAddress);
	pub fn get_sensor_config(address: SevenBitAddress);
	pub fn set_sensor_config(address: SevenBitAddress);
	pub fn get_metadata(address: SevenBitAddress);
	pub fn get_flash_record(address: SevenBitAddress);
	pub fn set_flash_record(address: SevenBitAddress);
	pub fn get_errors(address: SevenBitAddress);
	pub fn get_counts(address: SevenBitAddress);
	pub fn clear_counts(address: SevenBitAddress);
	pub fn set_tare(address: SevenBitAddress);
	pub fn clear_tare(address: SevenBitAddress);
	pub fn persist_tare(address: SevenBitAddress);
	pub fn set_reorientation(address: SevenBitAddress);
	pub fn reinitialize(address: SevenBitAddress);
	pub fn save_dynamic_calibration(address: SevenBitAddress);
	pub fn get_oscillator_type(address: SevenBitAddress);
	pub fn set_calibration_config(address: SevenBitAddress);
	pub fn get_calibration_config(address: SevenBitAddress);
	pub fn set_dynamic_calibration_auto_save(address: SevenBitAddress);
	pub fn flush(address: SevenBitAddress);
	pub fn clear_and_reset_dynamic_calibration(address: SevenBitAddress);
	pub fn start_calibration(address: SevenBitAddress);
	pub fn finish_calibration(address: SevenBitAddress);
	pub fn set_interactive_zro(address: SevenBitAddress);
}