use crate::utils::immediate::parse_immediate;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Handle I/O port instructions
pub fn handle_io_instruction(
    mnemonic: &str,
    operands: Option<&str>,
    constants: &HashMap<String, u16>,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    match mnemonic {
        "in" => {
            if let Some(ops) = operands {
                let parts: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(anyhow!("IN requires destination and port"));
                }

                let dest = parts[0];
                let port = parts[1];

                if dest == "a" && port.starts_with('(') && port.ends_with(')') {
                    let port_str = &port[1..port.len() - 1];

                    if port_str == "c" {
                        // IN A,(C)
                        result.push(0xed);
                        result.push(0x78);
                    } else {
                        // IN A,(n)
                        let port_num = parse_immediate(port_str, constants)?;
                        result.push(0xdb);
                        result.push((port_num & 0xff) as u8);
                    }
                    return Ok(Some(result));
                } else if port == "(c)" {
                    // IN r,(C)
                    result.push(0xed);
                    let opcode = match dest {
                        "b" => 0x40,
                        "c" => 0x48,
                        "d" => 0x50,
                        "e" => 0x58,
                        "h" => 0x60,
                        "l" => 0x68,
                        "a" => 0x78,
                        _ => return Err(anyhow!("Invalid register for IN r,(C): {}", dest)),
                    };
                    result.push(opcode);
                    return Ok(Some(result));
                }
            }
        },

        "out" => {
            if let Some(ops) = operands {
                let parts: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(anyhow!("OUT requires port and source"));
                }

                let port = parts[0];
                let src = parts[1];

                if port.starts_with('(') && port.ends_with(')') && src == "a" {
                    let port_str = &port[1..port.len() - 1];

                    if port_str == "c" {
                        // OUT (C),A
                        result.push(0xed);
                        result.push(0x79);
                    } else {
                        // OUT (n),A
                        let port_num = parse_immediate(port_str, constants)?;
                        result.push(0xd3);
                        result.push((port_num & 0xff) as u8);
                    }
                    return Ok(Some(result));
                } else if port == "(c)" {
                    // OUT (C),r
                    result.push(0xed);
                    let opcode = match src {
                        "b" => 0x41,
                        "c" => 0x49,
                        "d" => 0x51,
                        "e" => 0x59,
                        "h" => 0x61,
                        "l" => 0x69,
                        "a" => 0x79,
                        _ => return Err(anyhow!("Invalid register for OUT (C),r: {}", src)),
                    };
                    result.push(opcode);
                    return Ok(Some(result));
                }
            }
        },

        // Block I/O instructions
        "ini" => {
            result.push(0xed);
            result.push(0xa2);
            return Ok(Some(result));
        },
        "inir" => {
            result.push(0xed);
            result.push(0xb2);
            return Ok(Some(result));
        },
        "ind" => {
            result.push(0xed);
            result.push(0xaa);
            return Ok(Some(result));
        },
        "indr" => {
            result.push(0xed);
            result.push(0xba);
            return Ok(Some(result));
        },
        "outi" => {
            result.push(0xed);
            result.push(0xa3);
            return Ok(Some(result));
        },
        "otir" => {
            result.push(0xed);
            result.push(0xb3);
            return Ok(Some(result));
        },
        "outd" => {
            result.push(0xed);
            result.push(0xab);
            return Ok(Some(result));
        },
        "otdr" => {
            result.push(0xed);
            result.push(0xbb);
            return Ok(Some(result));
        },

        _ => {},
    }

    Ok(None)
}
