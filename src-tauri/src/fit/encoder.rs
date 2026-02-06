/// FIT file encoder.
///
/// Builds a complete FIT binary file with header, definition messages,
/// data messages, and CRC.

use super::crc::fit_crc;
use super::types::FieldDef;

/// FIT file encoder that writes messages sequentially.
pub struct FitEncoder {
    /// Accumulated file bytes (after header).
    data: Vec<u8>,
    /// Track which local message types have been defined.
    defined: [bool; 16],
}

impl FitEncoder {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(4096),
            defined: [false; 16],
        }
    }

    /// Write a definition message for a local message type.
    /// This tells the decoder what fields to expect in subsequent data messages.
    pub fn write_definition(
        &mut self,
        local_mesg_type: u8,
        global_mesg_num: u16,
        fields: &[FieldDef],
    ) {
        // Record header: bit 6 = 1 (definition), bits 0-3 = local type
        self.data.push(0x40 | (local_mesg_type & 0x0F));

        // Reserved byte
        self.data.push(0x00);

        // Architecture: 0 = little-endian
        self.data.push(0x00);

        // Global message number (little-endian)
        self.data.extend_from_slice(&global_mesg_num.to_le_bytes());

        // Number of fields
        self.data.push(fields.len() as u8);

        // Field definitions
        for field in fields {
            self.data.push(field.field_num);
            self.data.push(field.size);
            self.data.push(field.base_type);
        }

        self.defined[local_mesg_type as usize] = true;
    }

    /// Write a data message. Fields must match the most recent definition
    /// for this local message type.
    pub fn write_data(&mut self, local_mesg_type: u8, field_data: &[&[u8]]) {
        // Record header: bit 6 = 0 (data), bits 0-3 = local type
        self.data.push(local_mesg_type & 0x0F);

        for field in field_data {
            self.data.extend_from_slice(field);
        }
    }

    /// Write a definition + data message pair for convenience.
    pub fn write_message(
        &mut self,
        local_mesg_type: u8,
        global_mesg_num: u16,
        fields: &[FieldDef],
        field_data: &[&[u8]],
    ) {
        // Only write definition if not already defined for this local type
        if !self.defined[local_mesg_type as usize] {
            self.write_definition(local_mesg_type, global_mesg_num, fields);
        }
        self.write_data(local_mesg_type, field_data);
    }

    /// Write a definition + data message, always writing the definition
    /// (useful when the same local type is reused for different global messages).
    pub fn write_message_with_def(
        &mut self,
        local_mesg_type: u8,
        global_mesg_num: u16,
        fields: &[FieldDef],
        field_data: &[&[u8]],
    ) {
        self.write_definition(local_mesg_type, global_mesg_num, fields);
        self.write_data(local_mesg_type, field_data);
    }

    /// Finalize the FIT file: prepend header and append CRC.
    /// Returns the complete FIT file bytes.
    pub fn finish(self) -> Vec<u8> {
        let data_size = self.data.len() as u32;

        // 14-byte header
        let mut header = Vec::with_capacity(14);
        header.push(14); // Header size
        header.push(0x20); // Protocol version 2.0
        header.extend_from_slice(&(0x0867u16).to_le_bytes()); // Profile version 8.67
        header.extend_from_slice(&data_size.to_le_bytes()); // Data size
        header.extend_from_slice(b".FIT"); // Data type
        let header_crc = fit_crc(&header);
        header.extend_from_slice(&header_crc.to_le_bytes());

        // Combine header + data
        let mut file = Vec::with_capacity(14 + self.data.len() + 2);
        file.extend_from_slice(&header);
        file.extend_from_slice(&self.data);

        // File CRC (over header + data)
        let file_crc = fit_crc(&file);
        file.extend_from_slice(&file_crc.to_le_bytes());

        file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fit::types::*;

    #[test]
    fn test_encoder_produces_valid_header() {
        let encoder = FitEncoder::new();
        let bytes = encoder.finish();

        // Minimum size: 14 (header) + 0 (data) + 2 (CRC) = 16
        assert_eq!(bytes.len(), 16);

        // Check header
        assert_eq!(bytes[0], 14); // header size
        assert_eq!(&bytes[8..12], b".FIT"); // signature
    }

    #[test]
    fn test_definition_and_data_message() {
        let mut encoder = FitEncoder::new();

        let fields = vec![
            FieldDef::new(0, 1, BASE_TYPE_ENUM),   // type
            FieldDef::new(1, 2, BASE_TYPE_UINT16),  // manufacturer
        ];

        encoder.write_message(
            0,
            MESG_FILE_ID,
            &fields,
            &[
                &[FILE_TYPE_ACTIVITY],
                &MANUFACTURER_DEVELOPMENT.to_le_bytes(),
            ],
        );

        let bytes = encoder.finish();
        // Header(14) + def(1+1+1+2+1 + 2*3=6 = 12) + data(1+1+2=4) + CRC(2)
        assert_eq!(bytes.len(), 14 + 12 + 4 + 2);
    }
}
