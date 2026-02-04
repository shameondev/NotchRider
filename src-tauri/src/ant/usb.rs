use rusb::{Context, DeviceHandle, UsbContext};
use std::time::Duration;

// ANT+ USB Stick vendor/product IDs
const ANT_USB_VID: u16 = 0x0fcf; // Dynastream
const ANT_USB_PID: u16 = 0x1008; // ANT USB-m Stick

// USB endpoints for ANT+ stick
const ANT_USB_EP_OUT: u8 = 0x01;
const ANT_USB_EP_IN: u8 = 0x81;

// Timeouts
const USB_WRITE_TIMEOUT: Duration = Duration::from_millis(1000);
const USB_READ_TIMEOUT: Duration = Duration::from_millis(50); // Short timeout for non-blocking reads

pub struct AntUsb {
    context: Option<Context>,
    handle: Option<DeviceHandle<Context>>,
    found: bool,
}

impl AntUsb {
    pub fn new() -> Self {
        Self {
            context: None,
            handle: None,
            found: false,
        }
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
                self.context = Some(context);
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_found(&self) -> bool {
        self.found
    }

    pub fn open(&mut self) -> Result<(), String> {
        let context = self
            .context
            .as_ref()
            .ok_or("USB context not initialized. Call find_device first.")?;

        for device in context
            .devices()
            .map_err(|e| format!("Failed to list devices: {}", e))?
            .iter()
        {
            let desc = device
                .device_descriptor()
                .map_err(|e| format!("Failed to get descriptor: {}", e))?;

            if desc.vendor_id() == ANT_USB_VID && desc.product_id() == ANT_USB_PID {
                let mut handle = device
                    .open()
                    .map_err(|e| format!("Failed to open device: {}", e))?;

                // Detach kernel driver if necessary (Linux)
                #[cfg(target_os = "linux")]
                {
                    if handle.kernel_driver_active(0).unwrap_or(false) {
                        handle
                            .detach_kernel_driver(0)
                            .map_err(|e| format!("Failed to detach kernel driver: {}", e))?;
                    }
                }

                // Claim interface 0
                handle
                    .claim_interface(0)
                    .map_err(|e| format!("Failed to claim interface: {}", e))?;

                // Reset the device to ensure clean state
                handle
                    .reset()
                    .map_err(|e| format!("Failed to reset device: {}", e))?;

                self.handle = Some(handle);
                println!("ANT+ USB device opened successfully");
                return Ok(());
            }
        }

        Err("ANT+ device not found".to_string())
    }

    pub fn close(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.release_interface(0);
            println!("ANT+ USB device closed");
        }
    }

    pub fn write(&self, data: &[u8]) -> Result<usize, String> {
        let handle = self
            .handle
            .as_ref()
            .ok_or("Device not open. Call open() first.")?;

        handle
            .write_bulk(ANT_USB_EP_OUT, data, USB_WRITE_TIMEOUT)
            .map_err(|e| format!("USB write failed: {}", e))
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, String> {
        let handle = self
            .handle
            .as_ref()
            .ok_or("Device not open. Call open() first.")?;

        match handle.read_bulk(ANT_USB_EP_IN, buffer, USB_READ_TIMEOUT) {
            Ok(bytes) => Ok(bytes),
            Err(rusb::Error::Timeout) => Ok(0), // No data available
            Err(e) => Err(format!("USB read failed: {}", e)),
        }
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
