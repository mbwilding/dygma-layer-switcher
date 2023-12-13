use crate::config::Config;
use anyhow::{bail, Result};
use serialport::{SerialPort, SerialPortInfo};
use tracing::{debug, error};

pub fn configure(config: &Config) -> Result<Box<dyn SerialPort>> {
    let comm_port = config.comm_port.clone().unwrap_or_default();

    let port = match serialport::new(&comm_port, 115_200)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(std::time::Duration::from_millis(10))
        .open()
    {
        Ok(s) => s,
        Err(e) => {
            let msg = format!("Failed to open serial port: {} ({:?})", &comm_port, e);
            error!("{}", &msg);
            bail!("{}", &msg);
        }
    };

    Ok(port)
}

pub fn detect_ports() -> serialport::Result<Vec<SerialPortInfo>> {
    let ports = serialport::available_ports();
    debug!("Available ports: {:#?}", ports);

    ports
}
