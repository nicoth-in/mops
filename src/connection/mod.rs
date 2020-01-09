mod types;
pub use types::*;

mod bluetooth;
pub use bluetooth::*;

pub struct Ev3Adaptor {
    connector: Bluetooth,
    buffer: Vec<u8>,
    is_sent: bool,
}
impl Ev3Adaptor {
    /// Create new ev3 adaptor
    pub fn new() -> Self {
        Self {
            connector: Bluetooth::new(),
            buffer: Vec::new(),
            is_sent: false,
        }
    }
    pub fn end(mut self) {
        self.connector.stop();
    }
}
impl Write for Ev3Adaptor {
    /// Write by connnector
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer = buf.to_vec();
        self.is_sent = false;
        self.connector.send(&mut self.buffer);
        self.is_sent = true;
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
impl Read for Ev3Adaptor {
    fn read(&mut self, buf: &mut [u8]) ->std::io::Result<usize> {
        let bf = self.connector.read(8);
        Ok(buf.len())
    }
}

pub struct Ev3Api {
    adaptor: Ev3Adaptor,
    message: Vec<u8>,
}
impl Ev3Api {
    pub fn new() -> Self {
        Self {
            adaptor: Ev3Adaptor::new(),
            message: Vec::new(),
        }
    }
    pub fn end(mut self) {
        // Safe close all
        self.adaptor.flush().unwrap();
        self.adaptor.end();
    }
    pub fn bip(&mut self) {
        self.message = vec![0x0F, 0x00, 0x00, 0x01, 0x80, 0x00, 0x00, 0x94, 0x01, 0x81, 0x02, 0x82, 0xE8, 0x03, 0x82, 0xE8, 0x03];
        self.adaptor.write(&self.message);
    }
}
