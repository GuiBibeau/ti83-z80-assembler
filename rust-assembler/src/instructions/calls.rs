use crate::utils::immediate::parse_immediate;
use anyhow::Result;
use std::collections::HashMap;

pub fn handle_call_instruction(
    mnemonic: &str,
    operands: Option<&str>,
    labels: &HashMap<String, u16>,
    constants: &HashMap<String, u16>,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    match mnemonic {
        "call" => {
            if let Some(ops) = operands {
                // Handle conditional calls
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

                // Encode conditional call
                match condition {
                    Some("nz") => result.push(0xc4),
                    Some("z") => result.push(0xcc),
                    Some("nc") => result.push(0xd4),
                    Some("c") => result.push(0xdc),
                    Some("po") => result.push(0xe4),
                    Some("pe") => result.push(0xec),
                    Some("p") => result.push(0xf4),
                    Some("m") => result.push(0xfc),
                    None => result.push(0xcd), // Unconditional CALL
                    _ => return Ok(None),
                }

                result.push((address & 0xff) as u8);
                result.push(((address >> 8) & 0xff) as u8);
                return Ok(Some(result));
            }
        },
        "ret" => {
            if let Some(condition) = operands {
                // Conditional returns
                match condition.trim() {
                    "nz" => result.push(0xc0),
                    "z" => result.push(0xc8),
                    "nc" => result.push(0xd0),
                    "c" => result.push(0xd8),
                    "po" => result.push(0xe0),
                    "pe" => result.push(0xe8),
                    "p" => result.push(0xf0),
                    "m" => result.push(0xf8),
                    _ => return Ok(None),
                }
                return Ok(Some(result));
            }
            // Unconditional RET handled by main opcode table
        },
        _ => {},
    }

    Ok(None)
}
