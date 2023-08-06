use nannou::prelude::*;
// #[derive(Debug, PartialEq)]
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString, IntoAsciiString};

pub struct SerialHandler {
    port_name:      String,
    port:           Option<SerialPort>,
    connected:      bool,
    print_activity: bool,
    baudrate:       u32,
}
use serial2::SerialPort;

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
        for path in SerialPort::available_ports().unwrap() {
            println!("{:?}", path);
        }
    }

    pub fn write(&mut self, ascii: &AsciiStr) {
        // if self.print_activity {
        //     print!("{}", ascii);
        // }

        self.open_port();

        if let Some(port) = &self.port {
            port.write(ascii.as_bytes());
        }
    }

    pub fn open_port(&mut self) {
        match SerialPort::open(&self.port_name, self.baudrate) {
            Ok(p) =>
                if !self.connected {
                    self.port = Some(p);

                    self.connected = true;
                    if self.print_activity {
                        println!("port connected");
                    }
                },
            Err(_Err) => {
                self.connected = false;

                if self.print_activity {
                    println!("couldn't open port: {}", { _Err });
                }
            }
        }
    }
}
