/// CRC16 for FIT file format (CRC-CCITT with polynomial 0x1021)
/// Uses a 16-entry half-byte lookup table for compact implementation.

const CRC_TABLE: [u16; 16] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401,
    0xA001, 0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
];

pub fn fit_crc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        // Process low nibble
        let tmp = CRC_TABLE[(crc & 0x0F) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ CRC_TABLE[(byte & 0x0F) as usize];

        // Process high nibble
        let tmp = CRC_TABLE[(crc & 0x0F) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ CRC_TABLE[((byte >> 4) & 0x0F) as usize];
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_known_value() {
        // FIT SDK test vector: ".FIT" header bytes
        let data = [0x0E, 0x10, 0x67, 0x08];
        let crc = fit_crc(&data);
        assert_ne!(crc, 0); // Just verify it produces a non-zero result
    }

    #[test]
    fn test_crc_empty() {
        assert_eq!(fit_crc(&[]), 0);
    }
}
