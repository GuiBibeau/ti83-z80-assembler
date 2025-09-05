use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub fn parse_immediate(value: &str, constants: &HashMap<String, u16>) -> Result<u16> {
    let value = value.trim();

    // Check if it's a constant
    if let Some(&const_val) = constants.get(value) {
        return Ok(const_val);
    }

    // Check for character literal
    if value.starts_with('\'') && value.ends_with('\'') && value.len() == 3 {
        return Ok(value.chars().nth(1).unwrap() as u16);
    }

    // Parse as number
    if value.starts_with("$") || value.starts_with("0x") {
        // Hexadecimal
        let hex_str = if value.starts_with("$") {
            &value[1..]
        } else {
            &value[2..]
        };
        u16::from_str_radix(hex_str, 16).map_err(|e| anyhow!("Invalid hex number {}: {}", value, e))
    } else if value.starts_with("%") || value.starts_with("0b") {
        // Binary
        let bin_str = if value.starts_with("%") {
            &value[1..]
        } else {
            &value[2..]
        };
        u16::from_str_radix(bin_str, 2)
            .map_err(|e| anyhow!("Invalid binary number {}: {}", value, e))
    } else {
        // Decimal or expression
        // Simple expression support (e.g., "2+2")
        if value.contains('+') {
            let parts: Vec<&str> = value.split('+').collect();
            if parts.len() == 2 {
                let left = parse_immediate(parts[0], constants)?;
                let right = parse_immediate(parts[1], constants)?;
                return Ok(left.wrapping_add(right));
            }
        } else if value.contains('-') && !value.starts_with('-') {
            let parts: Vec<&str> = value.split('-').collect();
            if parts.len() == 2 {
                let left = parse_immediate(parts[0], constants)?;
                let right = parse_immediate(parts[1], constants)?;
                return Ok(left.wrapping_sub(right));
            }
        }

        // Parse as decimal
        value
            .parse::<u16>()
            .or_else(|_| {
                // Try as signed i16 and convert
                value.parse::<i16>().map(|v| v as u16)
            })
            .map_err(|e| anyhow!("Invalid number {}: {}", value, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal() {
        let constants = HashMap::new();
        assert_eq!(parse_immediate("42", &constants).unwrap(), 42);
        assert_eq!(parse_immediate("0", &constants).unwrap(), 0);
        assert_eq!(parse_immediate("65535", &constants).unwrap(), 65535);
    }

    #[test]
    fn test_parse_hex() {
        let constants = HashMap::new();
        assert_eq!(parse_immediate("$FF", &constants).unwrap(), 0xFF);
        assert_eq!(parse_immediate("0x1234", &constants).unwrap(), 0x1234);
        assert_eq!(parse_immediate("$9D93", &constants).unwrap(), 0x9D93);
    }

    #[test]
    fn test_parse_binary() {
        let constants = HashMap::new();
        assert_eq!(parse_immediate("%11111111", &constants).unwrap(), 0xFF);
        assert_eq!(parse_immediate("0b1010", &constants).unwrap(), 0b1010);
    }

    #[test]
    fn test_parse_char() {
        let constants = HashMap::new();
        assert_eq!(parse_immediate("'A'", &constants).unwrap(), 65);
        assert_eq!(parse_immediate("'0'", &constants).unwrap(), 48);
    }

    #[test]
    fn test_parse_constant() {
        let mut constants = HashMap::new();
        constants.insert("SCREEN_WIDTH".to_string(), 96);
        assert_eq!(parse_immediate("SCREEN_WIDTH", &constants).unwrap(), 96);
    }
}
