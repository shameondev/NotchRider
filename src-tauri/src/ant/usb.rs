use rusb::{Context, UsbContext};

// ANT+ USB Stick vendor/product IDs
const ANT_USB_VID: u16 = 0x0fcf; // Dynastream
const ANT_USB_PID: u16 = 0x1008; // ANT USB-m Stick

pub struct AntUsb {
    found: bool,
}

impl AntUsb {
    pub fn new() -> Self {
        Self { found: false }
    }

    pub fn find_device(&mut self) -> Result<bool, String> {
        let context =
            Context::new().map_err(|e| format!("Failed to create USB context: {}", e))?;

        for device in context
            .devices()
            .map_err(|e| format!("Failed to list devices: {}", e))?
            .iter()
        {
            let desc = device
                .device_descriptor()
                .map_err(|e| format!("Failed to get descriptor: {}", e))?;

            if desc.vendor_id() == ANT_USB_VID && desc.product_id() == ANT_USB_PID {
                println!("Found ANT+ USB stick!");
                self.found = true;
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_found(&self) -> bool {
        self.found
    }

    pub fn list_usb_devices(&self) -> Result<Vec<String>, String> {
        let context =
            Context::new().map_err(|e| format!("Failed to create USB context: {}", e))?;

        let mut devices = Vec::new();

        for device in context
            .devices()
            .map_err(|e| format!("Failed to list devices: {}", e))?
            .iter()
        {
            let desc = device
                .device_descriptor()
                .map_err(|e| format!("Failed to get descriptor: {}", e))?;

            devices.push(format!(
                "VID:{:04x} PID:{:04x}",
                desc.vendor_id(),
                desc.product_id()
            ));
        }

        Ok(devices)
    }
}

impl Default for AntUsb {
    fn default() -> Self {
        Self::new()
    }
}
