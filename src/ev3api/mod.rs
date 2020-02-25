#![allow(dead_code)]
mod types;
pub use types::*;

mod bluetooth;
use bluetooth::*;

/// Adaptor wich provides I/O with Ev3
/// Used by Ev3API
pub struct Ev3Adaptor {
    connector: Bluetooth,
    buffer: Vec<u8>,
    is_sent: bool,
}
impl Ev3Adaptor {
    /// Create new Ev3Adaptor
    pub fn new(device: Option<String>) -> Self {
        Self {
            connector: Bluetooth::new(device),
            buffer: Vec::new(),
            is_sent: false,
        }
    }
    /// Close adaptor session
    pub fn end(mut self) {
        self.connector.stop();
    }
}
impl Write for Ev3Adaptor {
    /// Write message
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
    /// Read message
    fn read(&mut self, buf: &mut [u8]) ->std::io::Result<usize> {
        self.connector.read(8);
        Ok(buf.len())
    }
}

/// Operates with msgs via Ev3Adaptor
pub struct Ev3Api {
    adaptor: Ev3Adaptor,
    message: Vec<u8>,
    counter: usize,
}
impl Ev3Api {
    /// Generate new Ev3API and attach to the first connected EV3 via Bluetooth API
    pub fn new() -> Self {
        Self {
            adaptor: Ev3Adaptor::new(None),
            message: Vec::new(),
            counter: 0,
        }
    }
    /// Generate new Ev3API and attach to the EV3 via Bluetooth API defined by device name
    pub fn by_name<S: Into<String>>(device: S) -> Self {
        Self {
            adaptor: Ev3Adaptor::new(Some(device.into())),
            message: Vec::new(),
            counter: 0,
        }
    }
    /// Set msg len (do at the end only)
    fn set_len(&mut self) {
        self.message.insert(0, 0);
        self.message.insert(0, self.message.len() as u8 - 1);
    }
    /// Set counter of msgs
    fn set_counter(&mut self) {
        self.message.insert(0, self.counter as u8);
        self.message.insert(0, 0);
    }
    /// Create msg
    fn gen_msg(&mut self, typ: u8, cmd: u8) {
        self.message.append(&mut vec![typ, cmd]);
        self.counter += 1;
    }
    // Clear buffer
    fn clear_buf(&mut self) {
        self.message = Vec::new();
    }
    /// Safe close all
    pub fn end(mut self) {
        self.adaptor.flush().unwrap();
        self.adaptor.end();
    }
    pub fn send(&mut self) {
        self.set_counter();
        self.set_len();
        self.adaptor.write(&self.message).unwrap();
        self.clear_buf();
    }
    /// Force make "bip" signal on Ev3 to test connection
    pub fn bip(&mut self) {
        self.message = vec![0x0F, 0x00, 0x00, 0x01, 0x80, 0x00, 0x00, 0x94, 0x01, 0x81, 0x02, 0x82, 0xE8, 0x03, 0x82, 0xE8, 0x03];
        self.adaptor.write(&self.message).unwrap();
        self.clear_buf();
    }
    /// Write bytes to mail box
    pub fn write_mail_box_bytes(&mut self, name: String, message: &[u8]) -> &mut Self {
        self.gen_msg(0x81, 0x9E);
        let mut name = name.as_bytes().to_vec();
        self.message.push(name.len() as u8);
        name.push(0x00);
        self.message.append(&mut name);
        let mut message = message.to_vec();
        self.message.push(message.len() as u8);
        self.message.push(0x00);
        self.message.append(&mut message);
        return self;
    }
    /// Write f32 to mail box
    pub fn write_mail_box_f32(&mut self, name: String, num: f32) -> &mut Self {
        let mut vb = num.to_le_bytes().to_vec();
        vb.remove(0);
        self.write_mail_box_bytes(name, &vb[..])
    }
}