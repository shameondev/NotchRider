// ANT+ Protocol Modules
//
// This module provides ANT+ FE-C (Fitness Equipment Control) protocol
// support for communicating with smart trainers.

pub mod channel;
pub mod fec;
pub mod hrm;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trainer_data_default() {
        let data = TrainerData::default();
        assert_eq!(data.power, 0);
        assert_eq!(data.speed, 0.0);
        assert_eq!(data.cadence, 0);
        assert_eq!(data.heart_rate, 0);
    }

    #[test]
    fn test_trainer_data_new() {
        let data = TrainerData::new(150, 32.5, 90, 140);
        assert_eq!(data.power, 150);
        assert_eq!(data.speed, 32.5);
        assert_eq!(data.cadence, 90);
        assert_eq!(data.heart_rate, 140);
    }

    #[test]
    fn test_is_active_with_power() {
        let data = TrainerData::new(100, 0.0, 0, 0);
        assert!(data.is_active());
    }

    #[test]
    fn test_is_active_with_speed() {
        let data = TrainerData::new(0, 15.0, 0, 0);
        assert!(data.is_active());
    }

    #[test]
    fn test_is_active_with_cadence() {
        let data = TrainerData::new(0, 0.0, 80, 0);
        assert!(data.is_active());
    }

    #[test]
    fn test_is_not_active_when_idle() {
        let data = TrainerData::default();
        assert!(!data.is_active());
    }

    #[test]
    fn test_is_not_active_with_only_heart_rate() {
        // Heart rate alone doesn't mean the trainer is active
        let data = TrainerData::new(0, 0.0, 0, 120);
        assert!(!data.is_active());
    }
}
