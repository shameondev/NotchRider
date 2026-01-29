// ANT+ Protocol Modules
//
// This module provides ANT+ FE-C (Fitness Equipment Control) protocol
// support for communicating with smart trainers.

pub mod channel;
pub mod fec;
pub mod usb;

use serde::{Deserialize, Serialize};

/// Real-time data from a connected trainer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerData {
    /// Instantaneous power in watts
    pub power: u16,
    /// Speed in km/h
    pub speed: f32,
    /// Cadence in RPM
    pub cadence: u8,
    /// Heart rate in BPM (from ANT+ HR strap if paired)
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

impl TrainerData {
    /// Create TrainerData with specific values (useful for testing)
    pub fn new(power: u16, speed: f32, cadence: u8, heart_rate: u8) -> Self {
        Self {
            power,
            speed,
            cadence,
            heart_rate,
        }
    }

    /// Check if the trainer is producing meaningful data
    pub fn is_active(&self) -> bool {
        self.power > 0 || self.speed > 0.0 || self.cadence > 0
    }
}
