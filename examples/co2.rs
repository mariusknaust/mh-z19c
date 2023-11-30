use linux_embedded_hal as hal;

use clap::Parser as _;
use hal::serial_core::SerialPort as _;
use embedded_timeout_macros::embedded_hal::timer::CountDown as _;

/// Read co2 measurements in ppm from a MH-Z19c connected via a serial port
#[derive(Debug, clap::Parser)]
#[command(name = "co2 measurement")]
struct Options
{
	/// Serial port device
	#[clap(short, long)]
	device: std::path::PathBuf,
	/// Timeout for waiting on a measurement
	#[clap(short, long, value_parser = humantime::parse_duration, default_value = "500ms")]
	timeout: std::time::Duration,
}

fn main()
{
	let options = Options::parse();

	let mut tty_port = hal::serial_unix::TTYPort::open(&options.device)
		.expect("Failed to open serial port device");

	tty_port.set_timeout(std::time::Duration::from_millis(100))
		.expect("Failed to set serial port timeout");
	tty_port.configure(
			&hal::serial_core::PortSettings
			{
				baud_rate: hal::serial_core::BaudRate::Baud9600,
				char_size: hal::serial_core::CharSize::Bits8,
				parity: hal::serial_core::Parity::ParityNone,
				stop_bits: hal::serial_core::StopBits::Stop1,
				flow_control: hal::serial_core::FlowControl::FlowNone,
			})
		.expect("Failed to configure serial port");

	let serial = hal::Serial(tty_port);
	let mut co2_sensor = mh_z19c::MhZ19C::new(serial);

	let mut timer = hal::SysTimer::new();
	timer.start(options.timeout);
	let co2_value = embedded_timeout_macros::block_timeout!(&mut timer, co2_sensor.read_co2_ppm())
		.expect("Failed to read CO₂ value");

	println!("{co2_value}");
}
