pub mod usb;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerData {
    pub power: u16,
    pub speed: f32,
    pub cadence: u8,
    pub heart_rate: u8,
}

impl Default for TrainerData {
    fn default() -> Self {
        Self {
            power: 0,
            speed: 0.0,
            cadence: 0,
            heart_rate: 0,
        }
    }
}
