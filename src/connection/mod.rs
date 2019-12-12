mod types;
pub use types::*;

mod bluetooth;
pub use bluetooth::*;

pub enum Ev3Connection {
    Bluetooth,
    Usb,
}
pub struct Ev3Adaptor {
    connector: Ev3Connection,
    buffer: Vec<u8>,
    is_sent: bool,
}
impl Ev3Adaptor {
    /// Create new ev3 adaptor
    pub fn new(con: Ev3Connection) -> Self {
        Self {
            connector: con,
            buffer: Vec::new(),
            is_sent: false,
        }
    }
}
impl Write for Ev3Adaptor {
    /// Write by connnector
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer = buf.to_vec();
        self.is_sent = false;
        match self.connector {
            Ev3Connection::Bluetooth => {
                unsafe {
                    startup();
                    //jungle();

                }
                self.is_sent = true;
            },
            Ev3Connection::Usb => {
                // ...
            },
        }
        Ok(0)
    }
    /// Flush buffer if sent
    fn flush(&mut self) -> std::io::Result<()> {
        if !self.is_sent {
            use std::io::*;
            Err(Error::new(ErrorKind::Other, "Can't flush, buffer is not sent"))
        } else {
            self.buffer = Vec::new();
            Ok(())
        }
    }
}
pub struct Ev3Api {
    adaptor: Ev3Adaptor,
    message: Vec<u8>,
}
impl Ev3Api {
    pub fn new(con: Ev3Connection) -> Self {
        Self {
            adaptor: Ev3Adaptor::new(con),
            message: Vec::new(),
        }
    }
    pub fn test(&mut self) {
        self.message = vec![0x0F, 0x00, 0x00, 0x01, 0x80, 0x00, 0x00, 0x94, 0x01, 0x81, 0x02, 0x82, 0xE8, 0x03, 0x82, 0xE8, 0x03];
        self.adaptor.write(&self.message);
    }
}
