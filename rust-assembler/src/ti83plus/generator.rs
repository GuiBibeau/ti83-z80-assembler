pub struct TI8XPGenerator;

impl TI8XPGenerator {
    pub fn create_8xp(program_name: &str, code: &[u8]) -> Vec<u8> {
        // TI-83 Plus file header
        let header = vec![
            0x2a, 0x2a, 0x54, 0x49, 0x38, 0x33, 0x46, 0x2a, // **TI83F*
            0x1a, 0x0a, 0x00, // signature
        ];

        // Comment (42 bytes)
        let mut comment = vec![0u8; 42];
        let comment_text = "Created by Z80 Assembler";
        for (i, ch) in comment_text.bytes().enumerate().take(42) {
            comment[i] = ch;
        }

        // Build variable header
        let data_length = code.len() + 2;
        let mut var_header = vec![0u8; 17];

        // Data section length (little-endian)
        var_header[0] = ((data_length + 13) & 0xff) as u8;
        var_header[1] = (((data_length + 13) >> 8) & 0xff) as u8;

        // Variable header
        var_header[2] = 0x0d;
        var_header[3] = 0x00;
        var_header[4] = (data_length & 0xff) as u8;
        var_header[5] = ((data_length >> 8) & 0xff) as u8;
        var_header[6] = 0x05; // Program type

        // Program name (8 bytes, padded)
        let name = program_name.to_uppercase();
        let name_bytes = name.as_bytes();
        for i in 0..8 {
            if i < name_bytes.len() {
                var_header[7 + i] = name_bytes[i];
            } else {
                var_header[7 + i] = 0;
            }
        }

        // Version and flag
        var_header[15] = 0x00;
        var_header[16] = 0x00;

        // Additional length bytes
        let length_bytes = vec![
            (data_length & 0xff) as u8,
            ((data_length >> 8) & 0xff) as u8,
        ];

        // Program data (size + code)
        let mut program_data = Vec::with_capacity(code.len() + 2);
        program_data.push((code.len() & 0xff) as u8);
        program_data.push(((code.len() >> 8) & 0xff) as u8);
        program_data.extend_from_slice(code);

        // Calculate checksum
        let mut checksum: u16 = 0;
        for i in 2..var_header.len() {
            checksum = checksum.wrapping_add(var_header[i] as u16);
        }
        checksum = checksum.wrapping_add(length_bytes[0] as u16);
        checksum = checksum.wrapping_add(length_bytes[1] as u16);
        for byte in &program_data {
            checksum = checksum.wrapping_add(*byte as u16);
        }

        // Build complete file
        let mut result = Vec::new();
        result.extend_from_slice(&header);
        result.extend_from_slice(&comment);
        result.extend_from_slice(&var_header);
        result.extend_from_slice(&length_bytes);
        result.extend_from_slice(&program_data);
        result.push((checksum & 0xff) as u8);
        result.push(((checksum >> 8) & 0xff) as u8);

        result
    }
}
