use std::time::Duration;

use nannou::prelude::*;
// #[derive(Debug, PartialEq)]
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString, IntoAsciiString};
use serialport::SerialPort;

pub struct SerialHandler {
    port_name:      String,
    port:           Option<Box<dyn SerialPort>>,
    connected:      bool,
    print_activity: bool,
    baudrate:       u32,
}

impl SerialHandler {
    pub fn new(port_name: &str, baudrate: u32, print_activity: bool) -> SerialHandler {
        SerialHandler {
            port: None,
            connected: false,
            print_activity,
            port_name: port_name.to_owned(),
            baudrate,
        }
    }

    pub fn print_avaliable_ports() {
        let ports = serialport::available_ports().expect("No ports found!");
        for p in ports {
            println!("{}", p.port_name);
        }
    }

    pub fn write(&mut self, ascii: &AsciiStr) {
        // if self.print_activity {
        //     print!("{}", ascii);
        // }

        let mut port = self.open_port();

        match port {
            Ok(mut port) => {
                let output = ascii.as_bytes();
                port.write(output);

                let mut serial_buf: Vec<u8> = vec![0; 32];
                let read = port.read(serial_buf.as_mut_slice());
                if let Ok(_) = read {
                    match String::from_utf8(serial_buf) {
                        Ok(v) => println!("{}", v),
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                }
            }
            Err(e) => println!("Failed to open port: {}", e),
        }
    }

    pub fn open_port(&mut self) -> Result<Box<dyn SerialPort>, serialport::Error> {
        serialport::new(&self.port_name, self.baudrate).timeout(Duration::from_millis(10)).open()
    }
}
