use crate::utils::immediate::parse_immediate;
use anyhow::Result;
use std::collections::HashMap;

pub fn handle_jump_instruction(
    mnemonic: &str,
    operands: Option<&str>,
    labels: &HashMap<String, u16>,
    constants: &HashMap<String, u16>,
    current_address: u16,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    match mnemonic {
        "jp" => {
            if let Some(ops) = operands {
                // Handle conditional jumps
                let (condition, target) = if ops.contains(',') {
                    let parts: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();
                    (Some(parts[0]), parts[1])
                } else {
                    (None, ops.trim())
                };

                let address = if let Some(&label_addr) = labels.get(target) {
                    label_addr
                } else {
                    parse_immediate(target, constants)?
                };

                // Encode conditional jump
                match condition {
                    Some("nz") => result.push(0xc2),
                    Some("z") => result.push(0xca),
                    Some("nc") => result.push(0xd2),
                    Some("c") => result.push(0xda),
                    Some("po") => result.push(0xe2),
                    Some("pe") => result.push(0xea),
                    Some("p") => result.push(0xf2),
                    Some("m") => result.push(0xfa),
                    None => result.push(0xc3), // Unconditional JP
                    _ => return Ok(None),
                }

                result.push((address & 0xff) as u8);
                result.push(((address >> 8) & 0xff) as u8);
                return Ok(Some(result));
            }
        },
        "jr" => {
            if let Some(ops) = operands {
                // Handle conditional relative jumps
                let (condition, target) = if ops.contains(',') {
                    let parts: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();
                    (Some(parts[0]), parts[1])
                } else {
                    (None, ops.trim())
                };

                let target_addr = if let Some(&label_addr) = labels.get(target) {
                    label_addr
                } else {
                    parse_immediate(target, constants)?
                };

                // Calculate relative offset
                // JR instruction is 2 bytes, offset is from the next instruction
                let offset = (target_addr as i32) - (current_address as i32 + 2);
                if offset < -128 || offset > 127 {
                    return Err(anyhow::anyhow!("JR target out of range: offset {}", offset));
                }

                // Encode conditional relative jump
                match condition {
                    Some("nz") => result.push(0x20),
                    Some("z") => result.push(0x28),
                    Some("nc") => result.push(0x30),
                    Some("c") => result.push(0x38),
                    None => result.push(0x18), // Unconditional JR
                    _ => return Ok(None),
                }

                result.push(offset as u8);
                return Ok(Some(result));
            }
        },
        "djnz" => {
            if let Some(target) = operands {
                let target_addr = if let Some(&label_addr) = labels.get(target.trim()) {
                    label_addr
                } else {
                    parse_immediate(target.trim(), constants)?
                };

                // Calculate relative offset
                let offset = (target_addr as i32) - (current_address as i32 + 2);
                if offset < -128 || offset > 127 {
                    return Err(anyhow::anyhow!(
                        "DJNZ target out of range: offset {}",
                        offset
                    ));
                }

                result.push(0x10); // DJNZ
                result.push(offset as u8);
                return Ok(Some(result));
            }
        },
        _ => {},
    }

    Ok(None)
}
