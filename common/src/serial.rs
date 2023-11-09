use crate::config::Config;
use serialport::SerialPort;
use tracing::error;

pub fn configure(config: &Config) -> Result<Box<dyn SerialPort>, bool> {
    let port = match serialport::new(config.comm_port.clone(), 9_600)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(std::time::Duration::from_millis(10))
        .open()
    {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Failed to open serial port: {} ({:#?})",
                &config.comm_port, e
            );
            return Err(true);
        }
    };

    Ok(port)
}
