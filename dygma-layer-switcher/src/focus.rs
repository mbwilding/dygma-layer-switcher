use anyhow::{anyhow, bail, Result};
use log::{error, info, trace};
use serialport::{SerialPort, SerialPortType};
use std::io::Write;
use std::time::Duration;

#[derive(Debug)]
pub struct SupportedDevice {
    pub name: &'static str,
    pub vendor_id: u16,
    pub product_id: u16,
}

impl SupportedDevice {
    pub const fn new(name: &'static str, vendor_id: u16, product_id: u16) -> Self {
        SupportedDevice {
            name,
            vendor_id,
            product_id,
        }
    }
}

pub const DEVICES: [SupportedDevice; 4] = [
    SupportedDevice::new("Defy Wired", 0x35ef, 0x0010),
    SupportedDevice::new("Defy Wireless", 0x35ef, 0x0012),
    SupportedDevice::new("Raise ANSI", 0x1209, 0x2201),
    SupportedDevice::new("Raise ISO", 0x1209, 0x2201),
];

#[derive(Debug, Clone)]
pub struct Device {
    pub name: &'static str,
    pub port: String,
}

#[derive(Default)]
pub struct Focus {
    port: Option<Box<dyn SerialPort>>,
}

impl Focus {
    pub fn find_all(&self) -> Result<Vec<Device>> {
        let ports = match serialport::available_ports() {
            Ok(ports) => ports,
            Err(e) => {
                let err_msg = format!("Failed to enumerate serial ports: {:?}", e);
                error!("{}", err_msg);
                bail!(err_msg)
            }
        };

        trace!("Available serial ports: {:?}", ports);

        let found_devices: Vec<Device> = ports
            .into_iter()
            .filter_map(|port| match &port.port_type {
                SerialPortType::UsbPort(info) => DEVICES
                    .iter()
                    .find(|&device| device.vendor_id == info.vid && device.product_id == info.pid)
                    .map(|device| Device {
                        name: device.name,
                        port: port.port_name,
                    }),
                _ => None,
            })
            .collect();

        info!("Found devices: {:?}", found_devices);

        Ok(found_devices)
    }

    pub fn find_first(&self) -> Result<Device> {
        let devices = match self.find_all() {
            Ok(devices) => devices,
            Err(e) => {
                let err_msg = format!("No device found: {:?}", e);
                error!("{}", err_msg);
                bail!(err_msg)
            }
        };

        let device = devices.first().ok_or_else(|| {
            let err_msg = "No supported devices found";
            error!("{}", err_msg);
            anyhow!(err_msg)
        })?;

        Ok(device.clone())
    }

    pub fn open_via_port(&mut self, port: &str) -> Result<()> {
        let port_settings = serialport::new(port, 115_200)
            .data_bits(serialport::DataBits::Eight)
            .flow_control(serialport::FlowControl::None)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .timeout(Duration::from_secs(5));

        let port = port_settings.open().map_err(|e| {
            let err_msg = format!("Failed to open serial port: {} ({:?})", &port, e);
            error!("{}", err_msg);
            anyhow!(err_msg)
        })?;

        self.port = Some(port);

        Ok(())
    }

    pub fn command(&mut self, command: &str) -> Result<()> {
        if let Some(ref mut port) = self.port {
            port.write_all(format!("{}\n", command).as_bytes())?;

            Ok(())
        } else {
            Err(anyhow!("Serial port is not open"))
        }
    }

    pub fn layer_move_to(&mut self, layer: u8) -> Result<()> {
        self.command(&format!("layer.moveTo {}", layer))
    }
}
