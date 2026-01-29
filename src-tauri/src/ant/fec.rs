// ANT+ FE-C (Fitness Equipment Control) Protocol Parser
//
// This module implements parsing for ANT+ FE-C data pages commonly
// transmitted by smart trainers and fitness equipment.
//
// Reference: ANT+ Fitness Equipment Device Profile

use super::TrainerData;

// FE-C Data Page Numbers
pub const PAGE_GENERAL_FE_DATA: u8 = 0x10; // 16
pub const PAGE_GENERAL_SETTINGS: u8 = 0x11; // 17
pub const PAGE_GENERAL_FE_METABOLIC: u8 = 0x12; // 18
pub const PAGE_SPECIFIC_TRAINER_DATA: u8 = 0x19; // 25
pub const PAGE_TRAINER_TORQUE: u8 = 0x1A; // 26
pub const PAGE_FE_CAPABILITIES: u8 = 0x36; // 54
pub const PAGE_USER_CONFIG: u8 = 0x37; // 55
pub const PAGE_REQUEST_DATA: u8 = 0x46; // 70
pub const PAGE_COMMAND_STATUS: u8 = 0x47; // 71
pub const PAGE_MANUFACTURER_ID: u8 = 0x50; // 80
pub const PAGE_PRODUCT_INFO: u8 = 0x51; // 81

// Equipment Types (for Page 16)
pub const EQUIPMENT_TYPE_GENERAL: u8 = 16;
pub const EQUIPMENT_TYPE_TREADMILL: u8 = 19;
pub const EQUIPMENT_TYPE_ELLIPTICAL: u8 = 20;
pub const EQUIPMENT_TYPE_STATIONARY_BIKE: u8 = 21;
pub const EQUIPMENT_TYPE_ROWER: u8 = 22;
pub const EQUIPMENT_TYPE_CLIMBER: u8 = 23;
pub const EQUIPMENT_TYPE_NORDIC_SKIER: u8 = 24;
pub const EQUIPMENT_TYPE_TRAINER: u8 = 25;

// Trainer Status Bit Flags (for Page 25)
pub const TRAINER_STATUS_BICYCLE_POWER: u8 = 0x01;
pub const TRAINER_STATUS_CALIBRATION_REQUIRED: u8 = 0x02;
pub const TRAINER_STATUS_CALIBRATION_IN_PROGRESS: u8 = 0x04;

// FE State (from Page 16 byte 7)
pub const FE_STATE_ASLEEP: u8 = 1;
pub const FE_STATE_READY: u8 = 2;
pub const FE_STATE_IN_USE: u8 = 3;
pub const FE_STATE_FINISHED: u8 = 4;

/// Parse ANT+ FE-C data pages
pub struct FecParser {
    // Accumulated values for calculating deltas
    last_event_count: u8,
    last_accumulated_power: u16,
    last_distance: u8,
    last_elapsed_time: u8,
}

impl FecParser {
    pub fn new() -> Self {
        Self {
            last_event_count: 0,
            last_accumulated_power: 0,
            last_distance: 0,
            last_elapsed_time: 0,
        }
    }

    /// Parse broadcast data message and return structured data page
    ///
    /// The data buffer should contain the 8-byte payload from an ANT+ broadcast message.
    /// Byte 0 is the data page number.
    pub fn parse_broadcast(&mut self, data: &[u8]) -> Option<FecDataPage> {
        if data.len() < 8 {
            return None;
        }

        let page_number = data[0];

        match page_number {
            PAGE_GENERAL_FE_DATA => Self::parse_general_fe_data(data),
            PAGE_SPECIFIC_TRAINER_DATA => self.parse_specific_trainer_data(data),
            PAGE_TRAINER_TORQUE => Self::parse_trainer_torque(data),
            PAGE_MANUFACTURER_ID => Self::parse_manufacturer_id(data),
            PAGE_PRODUCT_INFO => Self::parse_product_info(data),
            PAGE_FE_CAPABILITIES => Self::parse_fe_capabilities(data),
            PAGE_COMMAND_STATUS => Self::parse_command_status(data),
            _ => Some(FecDataPage::Unknown {
                page_number,
                raw_data: data.to_vec(),
            }),
        }
    }

    /// Page 16 (0x10): General FE Data
    ///
    /// Contains basic information common to all fitness equipment:
    /// - Equipment type
    /// - Elapsed time
    /// - Distance traveled
    /// - Speed
    fn parse_general_fe_data(data: &[u8]) -> Option<FecDataPage> {
        let equipment_type = data[1] & 0x1F; // Lower 5 bits
        let elapsed_time = data[2]; // 0.25s units, rolls over at 64s
        let distance = data[3]; // meters, rolls over at 256m
        let speed_lsb = data[4];
        let speed_msb = data[5];
        let speed = u16::from_le_bytes([speed_lsb, speed_msb]); // 0.001 m/s

        let heart_rate = data[6]; // 0xFF = invalid

        let capabilities_state = data[7];
        let fe_state = (capabilities_state >> 4) & 0x07;
        let lap_toggle = (capabilities_state >> 7) & 0x01 == 1;
        let distance_enabled = (capabilities_state & 0x04) != 0;
        let speed_enabled = speed != 0xFFFF;

        Some(FecDataPage::GeneralFE {
            equipment_type,
            elapsed_time_quarter_sec: elapsed_time,
            distance_meters: distance,
            speed_mms: if speed_enabled { speed } else { 0 },
            heart_rate: if heart_rate == 0xFF {
                None
            } else {
                Some(heart_rate)
            },
            fe_state,
            lap_toggle,
            distance_enabled,
        })
    }

    /// Page 25 (0x19): Specific Trainer/Stationary Bike Data
    ///
    /// Contains trainer-specific data:
    /// - Event count (for power averaging)
    /// - Cadence
    /// - Accumulated power
    /// - Instantaneous power
    /// - Trainer status
    fn parse_specific_trainer_data(&mut self, data: &[u8]) -> Option<FecDataPage> {
        let update_event_count = data[1];
        let cadence = data[2]; // RPM, 0xFF = invalid

        let accumulated_power_lsb = data[3];
        let accumulated_power_msb = data[4];
        let accumulated_power = u16::from_le_bytes([accumulated_power_lsb, accumulated_power_msb]);

        // Instantaneous power is 12 bits (lower 4 bits of byte 6 + all of byte 5)
        let power_lsb = data[5];
        let power_msb_and_status = data[6];
        let power_msb = power_msb_and_status & 0x0F;
        let instantaneous_power = u16::from_le_bytes([power_lsb, power_msb]);

        // Trainer status in upper 4 bits of byte 6
        let trainer_status_bits = (power_msb_and_status >> 4) & 0x0F;

        // Flags byte 7
        let flags = data[7];
        let target_power_limits = flags & 0x07;

        // Calculate average power if we have previous values
        let avg_power = self.calculate_average_power(update_event_count, accumulated_power);

        // Update last values
        self.last_event_count = update_event_count;
        self.last_accumulated_power = accumulated_power;

        Some(FecDataPage::SpecificTrainer {
            event_count: update_event_count,
            cadence: if cadence == 0xFF { None } else { Some(cadence) },
            accumulated_power,
            instantaneous_power,
            average_power: avg_power,
            trainer_status: TrainerStatus::from_bits(trainer_status_bits),
            target_power_limits,
        })
    }

    /// Calculate average power from accumulated values
    fn calculate_average_power(&self, event_count: u8, accumulated_power: u16) -> Option<u16> {
        if self.last_event_count == 0 && self.last_accumulated_power == 0 {
            return None;
        }

        // Handle rollover for event count (8-bit)
        let event_diff = event_count.wrapping_sub(self.last_event_count);
        if event_diff == 0 {
            return None;
        }

        // Handle rollover for accumulated power (16-bit)
        let power_diff = accumulated_power.wrapping_sub(self.last_accumulated_power);

        Some(power_diff / event_diff as u16)
    }

    /// Page 26 (0x1A): Trainer Torque Data
    fn parse_trainer_torque(data: &[u8]) -> Option<FecDataPage> {
        let update_event_count = data[1];
        let wheel_ticks = data[2];
        let wheel_period = u16::from_le_bytes([data[3], data[4]]);
        let accumulated_torque = u16::from_le_bytes([data[5], data[6]]);

        Some(FecDataPage::TrainerTorque {
            event_count: update_event_count,
            wheel_ticks,
            wheel_period,
            accumulated_torque,
        })
    }

    /// Page 80 (0x50): Manufacturer Identification
    fn parse_manufacturer_id(data: &[u8]) -> Option<FecDataPage> {
        // Bytes 1-2 are reserved
        let hw_revision = data[3];
        let manufacturer_id = u16::from_le_bytes([data[4], data[5]]);
        let model_number = u16::from_le_bytes([data[6], data[7]]);

        Some(FecDataPage::ManufacturerId {
            hw_revision,
            manufacturer_id,
            model_number,
        })
    }

    /// Page 81 (0x51): Product Information
    fn parse_product_info(data: &[u8]) -> Option<FecDataPage> {
        // Byte 1 is reserved
        let sw_revision_supplemental = data[2];
        let sw_revision_main = data[3];
        let serial_number = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

        Some(FecDataPage::ProductInfo {
            sw_revision_supplemental,
            sw_revision_main,
            serial_number,
        })
    }

    /// Page 54 (0x36): FE Capabilities
    fn parse_fe_capabilities(data: &[u8]) -> Option<FecDataPage> {
        // Bytes 1-4 are reserved
        let max_resistance = u16::from_le_bytes([data[5], data[6]]);
        let capabilities_bits = data[7];

        Some(FecDataPage::FECapabilities {
            max_resistance,
            supports_basic_resistance: (capabilities_bits & 0x01) != 0,
            supports_target_power: (capabilities_bits & 0x02) != 0,
            supports_simulation: (capabilities_bits & 0x04) != 0,
        })
    }

    /// Page 71 (0x47): Command Status
    fn parse_command_status(data: &[u8]) -> Option<FecDataPage> {
        let last_command = data[1];
        let sequence_number = data[2];
        let status = data[3];
        let data_bytes = [data[4], data[5], data[6], data[7]];

        Some(FecDataPage::CommandStatus {
            last_command,
            sequence_number,
            status,
            data: data_bytes,
        })
    }
}

impl Default for FecParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FecParser {
    /// Static method to parse a data page without needing accumulated state
    /// (useful for one-shot parsing from lib.rs)
    pub fn parse_data_page(data: &[u8]) -> Option<FecDataPage> {
        if data.len() < 8 {
            return None;
        }

        let page_number = data[0];

        match page_number {
            PAGE_GENERAL_FE_DATA => Self::parse_general_fe_data(data),
            PAGE_SPECIFIC_TRAINER_DATA => Self::parse_specific_trainer_static(data),
            PAGE_TRAINER_TORQUE => Self::parse_trainer_torque(data),
            PAGE_MANUFACTURER_ID => Self::parse_manufacturer_id(data),
            PAGE_PRODUCT_INFO => Self::parse_product_info(data),
            PAGE_FE_CAPABILITIES => Self::parse_fe_capabilities(data),
            PAGE_COMMAND_STATUS => Self::parse_command_status(data),
            _ => Some(FecDataPage::Unknown {
                page_number,
                raw_data: data.to_vec(),
            }),
        }
    }

    /// Static version of parse_specific_trainer_data (without average power calculation)
    fn parse_specific_trainer_static(data: &[u8]) -> Option<FecDataPage> {
        let update_event_count = data[1];
        let cadence = data[2];

        let accumulated_power_lsb = data[3];
        let accumulated_power_msb = data[4];
        let accumulated_power = u16::from_le_bytes([accumulated_power_lsb, accumulated_power_msb]);

        let power_lsb = data[5];
        let power_msb_and_status = data[6];
        let power_msb = power_msb_and_status & 0x0F;
        let instantaneous_power = u16::from_le_bytes([power_lsb, power_msb]);

        let trainer_status_bits = (power_msb_and_status >> 4) & 0x0F;
        let flags = data[7];
        let target_power_limits = flags & 0x07;

        Some(FecDataPage::SpecificTrainer {
            event_count: update_event_count,
            cadence: if cadence == 0xFF { None } else { Some(cadence) },
            accumulated_power,
            instantaneous_power,
            average_power: None, // Not calculated in static version
            trainer_status: TrainerStatus::from_bits(trainer_status_bits),
            target_power_limits,
        })
    }

    /// Static method to update TrainerData from a data page
    pub fn update_trainer_data(data: &mut TrainerData, page: &FecDataPage) {
        page.update_trainer_data(data);
    }
}

/// Trainer status decoded from Page 25
#[derive(Debug, Clone, Default)]
pub struct TrainerStatus {
    pub bicycle_power_calibration_required: bool,
    pub resistance_calibration_required: bool,
    pub user_config_required: bool,
}

impl TrainerStatus {
    pub fn from_bits(bits: u8) -> Self {
        Self {
            bicycle_power_calibration_required: (bits & 0x01) != 0,
            resistance_calibration_required: (bits & 0x02) != 0,
            user_config_required: (bits & 0x04) != 0,
        }
    }
}

/// Parsed FE-C data page
#[derive(Debug, Clone)]
pub enum FecDataPage {
    /// Page 16: General FE Data - basic equipment information
    GeneralFE {
        equipment_type: u8,
        elapsed_time_quarter_sec: u8,
        distance_meters: u8,
        speed_mms: u16, // 0.001 m/s
        heart_rate: Option<u8>,
        fe_state: u8,
        lap_toggle: bool,
        distance_enabled: bool,
    },

    /// Page 25: Specific Trainer Data - power and cadence
    SpecificTrainer {
        event_count: u8,
        cadence: Option<u8>,
        accumulated_power: u16,
        instantaneous_power: u16,
        average_power: Option<u16>,
        trainer_status: TrainerStatus,
        target_power_limits: u8,
    },

    /// Page 26: Trainer Torque Data
    TrainerTorque {
        event_count: u8,
        wheel_ticks: u8,
        wheel_period: u16,
        accumulated_torque: u16,
    },

    /// Page 54: FE Capabilities
    FECapabilities {
        max_resistance: u16,
        supports_basic_resistance: bool,
        supports_target_power: bool,
        supports_simulation: bool,
    },

    /// Page 71: Command Status
    CommandStatus {
        last_command: u8,
        sequence_number: u8,
        status: u8,
        data: [u8; 4],
    },

    /// Page 80: Manufacturer Identification
    ManufacturerId {
        hw_revision: u8,
        manufacturer_id: u16,
        model_number: u16,
    },

    /// Page 81: Product Information
    ProductInfo {
        sw_revision_supplemental: u8,
        sw_revision_main: u8,
        serial_number: u32,
    },

    /// Unknown page - stores raw data for debugging
    Unknown {
        page_number: u8,
        raw_data: Vec<u8>,
    },
}

impl FecDataPage {
    /// Update TrainerData from parsed page
    pub fn update_trainer_data(&self, data: &mut TrainerData) {
        match self {
            FecDataPage::GeneralFE {
                speed_mms,
                heart_rate,
                ..
            } => {
                // Convert 0.001 m/s to km/h: (m/s * 3.6 = km/h)
                // speed_mms is in 0.001 m/s, so divide by 1000 first
                data.speed = (*speed_mms as f32 / 1000.0) * 3.6;

                if let Some(hr) = heart_rate {
                    data.heart_rate = *hr;
                }
            }
            FecDataPage::SpecificTrainer {
                cadence,
                instantaneous_power,
                ..
            } => {
                if let Some(cad) = cadence {
                    data.cadence = *cad;
                }
                data.power = *instantaneous_power;
            }
            _ => {}
        }
    }

    /// Get human-readable equipment type name
    pub fn equipment_type_name(equipment_type: u8) -> &'static str {
        match equipment_type {
            EQUIPMENT_TYPE_GENERAL => "General",
            EQUIPMENT_TYPE_TREADMILL => "Treadmill",
            EQUIPMENT_TYPE_ELLIPTICAL => "Elliptical",
            EQUIPMENT_TYPE_STATIONARY_BIKE => "Stationary Bike",
            EQUIPMENT_TYPE_ROWER => "Rower",
            EQUIPMENT_TYPE_CLIMBER => "Climber",
            EQUIPMENT_TYPE_NORDIC_SKIER => "Nordic Skier",
            EQUIPMENT_TYPE_TRAINER => "Trainer",
            _ => "Unknown",
        }
    }

    /// Get human-readable FE state name
    pub fn fe_state_name(state: u8) -> &'static str {
        match state {
            FE_STATE_ASLEEP => "Asleep",
            FE_STATE_READY => "Ready",
            FE_STATE_IN_USE => "In Use",
            FE_STATE_FINISHED => "Finished",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_general_fe_data() {
        let mut parser = FecParser::new();

        // Simulated Page 16 data
        let data = [
            PAGE_GENERAL_FE_DATA, // Page number
            EQUIPMENT_TYPE_TRAINER, // Equipment type
            100,                  // Elapsed time (25 seconds)
            50,                   // Distance (50 meters)
            0xE8,
            0x03, // Speed: 1000 (1.0 m/s = 3.6 km/h)
            0xFF, // Heart rate invalid
            0x30, // State: In Use, no lap toggle
        ];

        let result = parser.parse_broadcast(&data);
        assert!(result.is_some());

        if let Some(FecDataPage::GeneralFE {
            equipment_type,
            speed_mms,
            heart_rate,
            fe_state,
            ..
        }) = result
        {
            assert_eq!(equipment_type, EQUIPMENT_TYPE_TRAINER);
            assert_eq!(speed_mms, 1000);
            assert!(heart_rate.is_none());
            assert_eq!(fe_state, FE_STATE_IN_USE);
        } else {
            panic!("Expected GeneralFE page");
        }
    }

    #[test]
    fn test_parse_specific_trainer_data() {
        let mut parser = FecParser::new();

        // Simulated Page 25 data
        let data = [
            PAGE_SPECIFIC_TRAINER_DATA, // Page number
            10,                         // Event count
            85,                         // Cadence: 85 RPM
            0x00,
            0x00, // Accumulated power LSB/MSB
            0xF4,
            0x01, // Power: 500W (0x01F4)
            0x00, // Flags
        ];

        let result = parser.parse_broadcast(&data);
        assert!(result.is_some());

        if let Some(FecDataPage::SpecificTrainer {
            cadence,
            instantaneous_power,
            ..
        }) = result
        {
            assert_eq!(cadence, Some(85));
            assert_eq!(instantaneous_power, 500);
        } else {
            panic!("Expected SpecificTrainer page");
        }
    }

    #[test]
    fn test_parse_manufacturer_id() {
        let mut parser = FecParser::new();

        // Simulated Page 80 data
        let data = [
            PAGE_MANUFACTURER_ID, // Page number
            0xFF,
            0xFF, // Reserved
            0x02, // HW revision
            0x01,
            0x00, // Manufacturer ID: 1 (Garmin)
            0x0A,
            0x00, // Model number: 10
        ];

        let result = parser.parse_broadcast(&data);
        assert!(result.is_some());

        if let Some(FecDataPage::ManufacturerId {
            hw_revision,
            manufacturer_id,
            model_number,
        }) = result
        {
            assert_eq!(hw_revision, 2);
            assert_eq!(manufacturer_id, 1);
            assert_eq!(model_number, 10);
        } else {
            panic!("Expected ManufacturerId page");
        }
    }

    #[test]
    fn test_update_trainer_data() {
        let mut trainer_data = TrainerData::default();

        let page = FecDataPage::SpecificTrainer {
            event_count: 1,
            cadence: Some(90),
            accumulated_power: 0,
            instantaneous_power: 250,
            average_power: None,
            trainer_status: TrainerStatus::default(),
            target_power_limits: 0,
        };

        page.update_trainer_data(&mut trainer_data);

        assert_eq!(trainer_data.cadence, 90);
        assert_eq!(trainer_data.power, 250);
    }

    #[test]
    fn test_speed_conversion() {
        let mut trainer_data = TrainerData::default();

        // 10000 mms = 10 m/s = 36 km/h
        let page = FecDataPage::GeneralFE {
            equipment_type: EQUIPMENT_TYPE_TRAINER,
            elapsed_time_quarter_sec: 0,
            distance_meters: 0,
            speed_mms: 10000,
            heart_rate: None,
            fe_state: FE_STATE_IN_USE,
            lap_toggle: false,
            distance_enabled: true,
        };

        page.update_trainer_data(&mut trainer_data);

        assert!((trainer_data.speed - 36.0).abs() < 0.01);
    }
}
