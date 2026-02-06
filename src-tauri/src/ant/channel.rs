// ANT+ Channel Management
//
// This module implements ANT+ channel configuration for communicating
// with fitness equipment devices using the FE-C protocol.

// ANT+ message types
const MESG_BROADCAST_DATA: u8 = 0x4E;
const MESG_CHANNEL_ID: u8 = 0x51;
const MESG_CHANNEL_FREQUENCY: u8 = 0x45;
const MESG_CHANNEL_PERIOD: u8 = 0x43;
const MESG_NETWORK_KEY: u8 = 0x46;
const MESG_ASSIGN_CHANNEL: u8 = 0x42;
const MESG_OPEN_CHANNEL: u8 = 0x4B;
const MESG_CLOSE_CHANNEL: u8 = 0x4C;
const MESG_SYSTEM_RESET: u8 = 0x4A;
const MESG_REQUEST_MESSAGE: u8 = 0x4D;

// ANT+ response message types
pub const MESG_RESPONSE_EVENT: u8 = 0x40;
pub const MESG_CHANNEL_STATUS: u8 = 0x52;
pub const MESG_CAPABILITIES: u8 = 0x54;

// ANT+ Network Key (public, same for all ANT+ devices)
const ANT_PLUS_NETWORK_KEY: [u8; 8] = [0xB9, 0xA5, 0x21, 0xFB, 0xBD, 0x72, 0xC3, 0x45];
const ANT_PLUS_RF_FREQUENCY: u8 = 57; // 2457 MHz (base 2400 + 57)

// ANT+ FE-C (Fitness Equipment) profile
const FEC_DEVICE_TYPE: u8 = 17; // Fitness Equipment
const FEC_CHANNEL_PERIOD: u16 = 8192; // 4Hz message rate (32768/8192 = 4)

// ANT+ HRM (Heart Rate Monitor) profile
const HRM_DEVICE_TYPE: u8 = 120; // Heart Rate Monitor
const HRM_CHANNEL_PERIOD: u16 = 8070; // ~4.06Hz message rate (32768/8070)

// Channel types
const CHANNEL_TYPE_SLAVE: u8 = 0x00; // Receive channel
const CHANNEL_TYPE_MASTER: u8 = 0x10; // Transmit channel

// Response/Event codes
pub const RESPONSE_NO_ERROR: u8 = 0x00;
pub const EVENT_RX_SEARCH_TIMEOUT: u8 = 0x01;
pub const EVENT_RX_FAIL: u8 = 0x02;
pub const EVENT_TX: u8 = 0x03;
pub const EVENT_TRANSFER_RX_FAILED: u8 = 0x04;
pub const EVENT_TRANSFER_TX_COMPLETED: u8 = 0x05;
pub const EVENT_TRANSFER_TX_FAILED: u8 = 0x06;
pub const EVENT_CHANNEL_CLOSED: u8 = 0x07;
pub const EVENT_RX_FAIL_GO_TO_SEARCH: u8 = 0x08;
pub const EVENT_CHANNEL_COLLISION: u8 = 0x09;

/// ANT+ sync byte that starts every message
pub const ANT_SYNC_BYTE: u8 = 0xA4;

/// ANT+ channel for FE-C communication
pub struct AntChannel {
    channel_number: u8,
    network_number: u8,
    device_number: u16,
    transmission_type: u8,
}

impl AntChannel {
    /// Create a new ANT+ channel with default settings for FE-C
    pub fn new(channel_number: u8) -> Self {
        Self {
            channel_number,
            network_number: 0,
            device_number: 0,     // Wildcard - search for any device
            transmission_type: 0, // Wildcard
        }
    }

    /// Create channel configured for a specific device
    pub fn with_device(channel_number: u8, device_number: u16, transmission_type: u8) -> Self {
        Self {
            channel_number,
            network_number: 0,
            device_number,
            transmission_type,
        }
    }

    /// Get the channel number
    pub fn channel_number(&self) -> u8 {
        self.channel_number
    }

    /// Build ANT message with sync byte, length, message ID, data, and checksum
    ///
    /// ANT message format:
    /// [SYNC][LENGTH][MSG_ID][DATA...][CHECKSUM]
    ///
    /// Where CHECKSUM = XOR of all preceding bytes
    pub fn build_message(msg_id: u8, data: &[u8]) -> Vec<u8> {
        let mut msg = vec![ANT_SYNC_BYTE, data.len() as u8, msg_id];
        msg.extend_from_slice(data);

        // Calculate checksum (XOR of all bytes)
        let checksum = msg.iter().fold(0u8, |acc, &b| acc ^ b);
        msg.push(checksum);

        msg
    }

    /// Parse an ANT message from raw bytes
    ///
    /// Returns (message_id, channel, data) if valid
    pub fn parse_message(buffer: &[u8]) -> Option<(u8, u8, Vec<u8>)> {
        if buffer.len() < 4 {
            return None;
        }

        // Find sync byte
        let sync_pos = buffer.iter().position(|&b| b == ANT_SYNC_BYTE)?;
        let buffer = &buffer[sync_pos..];

        if buffer.len() < 4 {
            return None;
        }

        let length = buffer[1] as usize;
        let msg_id = buffer[2];

        // Check if we have enough data
        if buffer.len() < 4 + length {
            return None;
        }

        // Verify checksum
        let checksum_pos = 3 + length;
        let calculated_checksum = buffer[..checksum_pos].iter().fold(0u8, |acc, &b| acc ^ b);
        if calculated_checksum != buffer[checksum_pos] {
            return None;
        }

        let channel = if length > 0 { buffer[3] } else { 0 };
        let data = buffer[3..3 + length].to_vec();

        Some((msg_id, channel, data))
    }

    /// System reset message - resets the ANT+ chip
    pub fn reset_system() -> Vec<u8> {
        Self::build_message(MESG_SYSTEM_RESET, &[0x00])
    }

    /// Set network key - required before opening channels
    pub fn set_network_key(&self) -> Vec<u8> {
        let mut data = vec![self.network_number];
        data.extend_from_slice(&ANT_PLUS_NETWORK_KEY);
        Self::build_message(MESG_NETWORK_KEY, &data)
    }

    /// Assign channel as slave (receive mode)
    pub fn assign_channel(&self) -> Vec<u8> {
        Self::build_message(
            MESG_ASSIGN_CHANNEL,
            &[
                self.channel_number,
                CHANNEL_TYPE_SLAVE,
                self.network_number,
            ],
        )
    }

    /// Assign channel as master (transmit mode)
    pub fn assign_channel_master(&self) -> Vec<u8> {
        Self::build_message(
            MESG_ASSIGN_CHANNEL,
            &[
                self.channel_number,
                CHANNEL_TYPE_MASTER,
                self.network_number,
            ],
        )
    }

    /// Set channel ID for device search
    ///
    /// Using device_number=0 and transmission_type=0 creates a wildcard
    /// search that will connect to any FE-C device.
    pub fn set_channel_id(&self) -> Vec<u8> {
        Self::build_message(
            MESG_CHANNEL_ID,
            &[
                self.channel_number,
                (self.device_number & 0xFF) as u8,
                (self.device_number >> 8) as u8,
                FEC_DEVICE_TYPE,
                self.transmission_type,
            ],
        )
    }

    /// Set RF frequency for ANT+ (2457 MHz)
    pub fn set_channel_frequency(&self) -> Vec<u8> {
        Self::build_message(
            MESG_CHANNEL_FREQUENCY,
            &[self.channel_number, ANT_PLUS_RF_FREQUENCY],
        )
    }

    /// Set channel ID for HRM device search
    pub fn set_channel_id_hrm(&self) -> Vec<u8> {
        Self::build_message(
            MESG_CHANNEL_ID,
            &[
                self.channel_number,
                (self.device_number & 0xFF) as u8,
                (self.device_number >> 8) as u8,
                HRM_DEVICE_TYPE,
                self.transmission_type,
            ],
        )
    }

    /// Set channel period for HRM (~4.06Hz)
    pub fn set_channel_period_hrm(&self) -> Vec<u8> {
        Self::build_message(
            MESG_CHANNEL_PERIOD,
            &[
                self.channel_number,
                (HRM_CHANNEL_PERIOD & 0xFF) as u8,
                (HRM_CHANNEL_PERIOD >> 8) as u8,
            ],
        )
    }

    /// Set channel period for 4Hz message rate
    pub fn set_channel_period(&self) -> Vec<u8> {
        Self::build_message(
            MESG_CHANNEL_PERIOD,
            &[
                self.channel_number,
                (FEC_CHANNEL_PERIOD & 0xFF) as u8,
                (FEC_CHANNEL_PERIOD >> 8) as u8,
            ],
        )
    }

    /// Open channel to begin receiving data
    pub fn open_channel(&self) -> Vec<u8> {
        Self::build_message(MESG_OPEN_CHANNEL, &[self.channel_number])
    }

    /// Close channel
    pub fn close_channel(&self) -> Vec<u8> {
        Self::build_message(MESG_CLOSE_CHANNEL, &[self.channel_number])
    }

    /// Request channel status
    pub fn request_channel_status(&self) -> Vec<u8> {
        Self::build_message(
            MESG_REQUEST_MESSAGE,
            &[self.channel_number, MESG_CHANNEL_STATUS],
        )
    }

    /// Request device capabilities
    pub fn request_capabilities(&self) -> Vec<u8> {
        Self::build_message(MESG_REQUEST_MESSAGE, &[0x00, MESG_CAPABILITIES])
    }

    /// Get the complete initialization sequence for FE-C
    ///
    /// Returns a vector of messages that should be sent in order
    /// with appropriate delays between them.
    pub fn get_init_sequence(&self) -> Vec<Vec<u8>> {
        vec![
            Self::reset_system(),
            self.set_network_key(),
            self.assign_channel(),
            self.set_channel_id(),
            self.set_channel_frequency(),
            self.set_channel_period(),
            self.open_channel(),
        ]
    }

    /// Get initialization sequence for HRM (no reset, assumes network key already set)
    ///
    /// Use this for a second channel when FE-C is already initialized
    pub fn get_hrm_init_sequence(&self) -> Vec<Vec<u8>> {
        vec![
            self.assign_channel(),
            self.set_channel_id_hrm(),
            self.set_channel_frequency(),
            self.set_channel_period_hrm(),
            self.open_channel(),
        ]
    }
}

impl Default for AntChannel {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_message_checksum() {
        // Test that checksum is correctly calculated
        let msg = AntChannel::build_message(MESG_SYSTEM_RESET, &[0x00]);
        assert_eq!(msg[0], ANT_SYNC_BYTE);
        assert_eq!(msg[1], 1); // Length
        assert_eq!(msg[2], MESG_SYSTEM_RESET);
        assert_eq!(msg[3], 0x00);
        // Checksum = 0xA4 ^ 0x01 ^ 0x4A ^ 0x00 = 0xEF
        assert_eq!(msg[4], 0xEF);
    }

    #[test]
    fn test_channel_id_message() {
        let channel = AntChannel::new(0);
        let msg = channel.set_channel_id();

        assert_eq!(msg[0], ANT_SYNC_BYTE);
        assert_eq!(msg[1], 5); // Length
        assert_eq!(msg[2], MESG_CHANNEL_ID);
        assert_eq!(msg[3], 0); // Channel number
        assert_eq!(msg[4], 0); // Device number LSB
        assert_eq!(msg[5], 0); // Device number MSB
        assert_eq!(msg[6], FEC_DEVICE_TYPE); // Device type
        assert_eq!(msg[7], 0); // Transmission type
    }

    #[test]
    fn test_parse_message() {
        let channel = AntChannel::new(0);
        let original = channel.set_channel_frequency();

        let parsed = AntChannel::parse_message(&original);
        assert!(parsed.is_some());

        let (msg_id, ch, data) = parsed.unwrap();
        assert_eq!(msg_id, MESG_CHANNEL_FREQUENCY);
        assert_eq!(ch, 0);
        assert_eq!(data.len(), 2);
        assert_eq!(data[1], ANT_PLUS_RF_FREQUENCY);
    }

    #[test]
    fn test_init_sequence_length() {
        let channel = AntChannel::new(0);
        let sequence = channel.get_init_sequence();
        assert_eq!(sequence.len(), 7); // Reset, network key, assign, ID, freq, period, open
    }
}
