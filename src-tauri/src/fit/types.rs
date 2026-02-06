/// FIT file format types, constants, and field definitions.

// FIT epoch: 1989-12-31 00:00:00 UTC
// Unix epoch offset to FIT epoch
pub const FIT_EPOCH_OFFSET: u32 = 631065600;

// Base types (used in field definitions)
pub const BASE_TYPE_ENUM: u8 = 0x00;
pub const BASE_TYPE_UINT8: u8 = 0x0D;
pub const BASE_TYPE_UINT16: u8 = 0x84;
pub const BASE_TYPE_UINT32: u8 = 0x86;
pub const BASE_TYPE_STRING: u8 = 0x07;

// Message numbers (Global Message Numbers)
pub const MESG_FILE_ID: u16 = 0;
pub const MESG_EVENT: u16 = 21;
pub const MESG_RECORD: u16 = 20;
pub const MESG_LAP: u16 = 19;
pub const MESG_SESSION: u16 = 18;
pub const MESG_ACTIVITY: u16 = 34;

// File type
pub const FILE_TYPE_ACTIVITY: u8 = 4;

// Manufacturer
pub const MANUFACTURER_DEVELOPMENT: u16 = 255;

// Product
pub const PRODUCT_NOTCHRIDER: u16 = 1;

// Sport
pub const SPORT_CYCLING: u8 = 2;
pub const SUB_SPORT_INDOOR_CYCLING: u8 = 6;

// Event
pub const EVENT_TIMER: u8 = 0;
pub const EVENT_TYPE_START: u8 = 0;
pub const EVENT_TYPE_STOP_ALL: u8 = 4;

// Lap trigger
pub const LAP_TRIGGER_SESSION_END: u8 = 7;

// Activity type
pub const ACTIVITY_MANUAL: u8 = 0;

/// A field definition within a FIT message.
#[derive(Clone)]
pub struct FieldDef {
    pub field_num: u8,
    pub size: u8,
    pub base_type: u8,
}

impl FieldDef {
    pub fn new(field_num: u8, size: u8, base_type: u8) -> Self {
        Self {
            field_num,
            size,
            base_type,
        }
    }
}

/// Convert a Unix timestamp (seconds since 1970) to FIT timestamp.
pub fn unix_to_fit_timestamp(unix_secs: u64) -> u32 {
    (unix_secs as u32).wrapping_sub(FIT_EPOCH_OFFSET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_timestamp_conversion() {
        // 2024-01-01 00:00:00 UTC = 1704067200 Unix
        let fit_ts = unix_to_fit_timestamp(1704067200);
        assert_eq!(fit_ts, 1704067200 - FIT_EPOCH_OFFSET);
    }
}
