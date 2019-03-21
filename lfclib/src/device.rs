use consts;
use hidapi::{HidResult, HidApi, HidDevice};

pub struct LuxaforDeviceDescriptor {
    pub vendor_id  : u16,
    pub product_id : u16
}

pub struct LuxaforContext {
    hid_api : HidApi
}

pub struct LuxaforDevice<'a> {
    hid_device : HidDevice<'a>
}

impl LuxaforContext {
    pub fn new() -> HidResult<LuxaforContext> {
        Ok(LuxaforContext {
            hid_api: HidApi::new()?
        })
    }

    pub fn open_device(&self, device_descriptor: LuxaforDeviceDescriptor) -> HidResult<LuxaforDevice> {
        LuxaforDevice::new(self.hid_api.open(device_descriptor.vendor_id, device_descriptor.product_id)?)
    }
}

impl<'a> LuxaforDevice<'a> {
    pub fn new(device: HidDevice) -> HidResult<LuxaforDevice> {
        Ok(LuxaforDevice {
            hid_device: device
        })
    }

    pub fn solid(&self, r: u8, g: u8, b: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::STATIC, consts::led::ALL, r, g, b])
    }

    pub fn raw(&self, mode: consts::mode::Mode, led: consts::led::Led, r: u8, g: u8, b: u8, extra: &[u8]) -> HidResult<usize> {
        let mut cmd : Vec<u8> = Vec::new();
        cmd.push(mode as u8);
        cmd.push(led as u8);
        cmd.extend_from_slice(&[r,g,b]);
        cmd.extend_from_slice(extra);
        // &[mode as u8, led as u8, r, g, b, x, y]
        self.hid_device.write(&cmd)
    }

    pub fn fade(&self, led: consts::led::Led, r: u8, g: u8, b: u8, speed: u8) -> HidResult<usize> {
        self.raw(consts::mode::Mode::Fade, led, r, g, b, &[speed, 0])
    }

    pub fn strobe(&self, led: consts::led::Led, r: u8, g: u8, b: u8, speed: u8, repeat: u8) -> HidResult<usize> {
        self.raw(consts::mode::Mode::Strobe, led, r, g, b, &[speed, 0, repeat])
    }

    pub fn wave(&self, wave: u8, r: u8, g: u8, b: u8, speed: u8, repeat: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::Mode::Wave as u8, wave, r,g,b,0,repeat,speed])
    }

    pub fn pattern(&self, pattern: u8, repeat: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::Mode::Pattern as u8, pattern, repeat, 0,0,0,0])
    }
}
