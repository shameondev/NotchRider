/// ANT+ Heart Rate Monitor profile parser
/// Device Type: 120 (0x78)
/// Channel Period: 8070
/// RF Frequency: 57

// HR Data Page numbers
pub const PAGE_0_DEFAULT: u8 = 0x00;
pub const PAGE_1_CUMULATIVE_TIME: u8 = 0x01;
pub const PAGE_2_MANUFACTURER: u8 = 0x02;
pub const PAGE_3_PRODUCT: u8 = 0x03;
pub const PAGE_4_PREVIOUS_HR: u8 = 0x04;

pub struct HrmParser;

impl HrmParser {
    /// Parse heart rate from ANT+ HR broadcast data
    /// Returns the computed heart rate (BPM) from byte 7
    pub fn parse_heart_rate(data: &[u8]) -> Option<u8> {
        if data.len() < 8 {
            return None;
        }

        // Byte 7 (index 7) contains the computed heart rate in all page formats
        let hr = data[7];

        // HR of 0 means invalid/no data
        if hr == 0 {
            return None;
        }

        Some(hr)
    }

    /// Get the page number from HR data
    pub fn get_page_number(data: &[u8]) -> Option<u8> {
        if data.is_empty() {
            return None;
        }
        // Page number is in bits 0-6 of byte 0
        Some(data[0] & 0x7F)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heart_rate() {
        // Typical HR broadcast with HR = 142 BPM
        let data = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 142];
        assert_eq!(HrmParser::parse_heart_rate(&data), Some(142));
    }

    #[test]
    fn test_parse_zero_heart_rate() {
        let data = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0];
        assert_eq!(HrmParser::parse_heart_rate(&data), None);
    }

    #[test]
    fn test_short_data() {
        let data = [0x00, 0xFF, 0xFF];
        assert_eq!(HrmParser::parse_heart_rate(&data), None);
    }
}
