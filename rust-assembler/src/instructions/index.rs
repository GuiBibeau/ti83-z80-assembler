use crate::utils::immediate::parse_immediate;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Handle IX/IY indexed operations like LD A,(IX+d)
///
/// IMPORTANT: The TI-83 Plus OS uses IX as a system flags pointer.
/// Using IX without preserving it can cause display corruption.
/// Programs should PUSH IX before use and POP IX after.
pub fn handle_index_instruction(
    mnemonic: &str,
    operands: Option<&str>,
    labels: &HashMap<String, u16>,
    constants: &HashMap<String, u16>,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    if let Some(ops) = operands {
        if mnemonic == "push" && ops == "ix" {
            result.push(0xdd);
            result.push(0xe5);
            return Ok(Some(result));
        } else if mnemonic == "push" && ops == "iy" {
            result.push(0xfd);
            result.push(0xe5);
            return Ok(Some(result));
        } else if mnemonic == "pop" && ops == "ix" {
            result.push(0xdd);
            result.push(0xe1);
            return Ok(Some(result));
        } else if mnemonic == "pop" && ops == "iy" {
            result.push(0xfd);
            result.push(0xe1);
            return Ok(Some(result));
        }

        if mnemonic == "ld" {
            if ops.starts_with("ix,") {
                let value_str = ops[3..].trim();
                if !value_str.starts_with('(') {
                    let value = if let Some(&label_addr) = labels.get(value_str) {
                        label_addr
                    } else {
                        parse_immediate(value_str, constants)?
                    };
                    result.push(0xdd);
                    result.push(0x21);
                    result.push((value & 0xff) as u8);
                    result.push(((value >> 8) & 0xff) as u8);
                    return Ok(Some(result));
                }
            } else if ops.starts_with("iy,") {
                let value_str = ops[3..].trim();
                if !value_str.starts_with('(') {
                    // LD IY,nn
                    let value = if let Some(&label_addr) = labels.get(value_str) {
                        label_addr
                    } else {
                        parse_immediate(value_str, constants)?
                    };
                    result.push(0xfd);
                    result.push(0x21);
                    result.push((value & 0xff) as u8);
                    result.push(((value >> 8) & 0xff) as u8);
                    return Ok(Some(result));
                }
            }

            if ops.contains("(ix+") || ops.contains("(ix-") || ops.contains("(ix)") {
                return handle_ix_indexed_load(ops, constants);
            } else if ops.contains("(iy+") || ops.contains("(iy-") || ops.contains("(iy)") {
                return handle_iy_indexed_load(ops, constants);
            }

            if ops.starts_with('(') && ops.contains("),ix") {
                let addr_str = &ops[1..ops.find(')').unwrap()];
                let address = parse_immediate(addr_str, constants)?;
                result.push(0xdd); // IX prefix
                result.push(0x22); // LD (nn),HL opcode
                result.push((address & 0xff) as u8);
                result.push(((address >> 8) & 0xff) as u8);
                return Ok(Some(result));
            } else if ops.starts_with('(') && ops.contains("),iy") {
                let addr_str = &ops[1..ops.find(')').unwrap()];
                let address = parse_immediate(addr_str, constants)?;
                result.push(0xfd); // IY prefix
                result.push(0x22); // LD (nn),HL opcode
                result.push((address & 0xff) as u8);
                result.push(((address >> 8) & 0xff) as u8);
                return Ok(Some(result));
            }

            if ops.starts_with("ix,(") && ops.ends_with(')') {
                let addr_str = &ops[4..ops.len() - 1];
                let address = parse_immediate(addr_str, constants)?;
                result.push(0xdd); // IX prefix
                result.push(0x2a); // LD HL,(nn) opcode
                result.push((address & 0xff) as u8);
                result.push(((address >> 8) & 0xff) as u8);
                return Ok(Some(result));
            } else if ops.starts_with("iy,(") && ops.ends_with(')') {
                let addr_str = &ops[4..ops.len() - 1];
                let address = parse_immediate(addr_str, constants)?;
                result.push(0xfd); // IY prefix
                result.push(0x2a); // LD HL,(nn) opcode
                result.push((address & 0xff) as u8);
                result.push(((address >> 8) & 0xff) as u8);
                return Ok(Some(result));
            }
        }

        if mnemonic == "inc" && ops == "ix" {
            result.push(0xdd); // IX prefix
            result.push(0x23); // INC HL opcode
            return Ok(Some(result));
        } else if mnemonic == "inc" && ops == "iy" {
            result.push(0xfd); // IY prefix
            result.push(0x23); // INC HL opcode
            return Ok(Some(result));
        } else if mnemonic == "dec" && ops == "ix" {
            result.push(0xdd); // IX prefix
            result.push(0x2b); // DEC HL opcode
            return Ok(Some(result));
        } else if mnemonic == "dec" && ops == "iy" {
            result.push(0xfd); // IY prefix
            result.push(0x2b); // DEC HL opcode
            return Ok(Some(result));
        }

        if mnemonic == "add" {
            if ops.starts_with("ix,") {
                let reg = ops[3..].trim();
                let opcode = match reg {
                    "bc" => 0x09,
                    "de" => 0x19,
                    "ix" => 0x29,
                    "sp" => 0x39,
                    _ => return Ok(None),
                };
                result.push(0xdd); // IX prefix
                result.push(opcode);
                return Ok(Some(result));
            } else if ops.starts_with("iy,") {
                let reg = ops[3..].trim();
                let opcode = match reg {
                    "bc" => 0x09,
                    "de" => 0x19,
                    "iy" => 0x29,
                    "sp" => 0x39,
                    _ => return Ok(None),
                };
                result.push(0xfd); // IY prefix
                result.push(opcode);
                return Ok(Some(result));
            }
        }

        if mnemonic == "ex" {
            if ops == "(sp),ix" {
                result.push(0xdd); // IX prefix
                result.push(0xe3); // EX (SP),HL opcode
                return Ok(Some(result));
            } else if ops == "(sp),iy" {
                result.push(0xfd); // IY prefix
                result.push(0xe3); // EX (SP),HL opcode
                return Ok(Some(result));
            }
        }

        if mnemonic == "jp" {
            if ops == "(ix)" {
                result.push(0xdd); // IX prefix
                result.push(0xe9); // JP (HL) opcode
                return Ok(Some(result));
            } else if ops == "(iy)" {
                result.push(0xfd); // IY prefix
                result.push(0xe9); // JP (HL) opcode
                return Ok(Some(result));
            }
        }

        if mnemonic == "ld" && ops == "sp,ix" {
            result.push(0xdd); // IX prefix
            result.push(0xf9); // LD SP,HL opcode
            return Ok(Some(result));
        } else if mnemonic == "ld" && ops == "sp,iy" {
            result.push(0xfd); // IY prefix
            result.push(0xf9); // LD SP,HL opcode
            return Ok(Some(result));
        }
    }

    Ok(None)
}

fn handle_ix_indexed_load(ops: &str, constants: &HashMap<String, u16>) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    // Parse the displacement from (IX+d) or (IX-d)
    let (reg, displacement) = parse_indexed_operand(ops, "ix", constants)?;

    result.push(0xdd); // IX prefix

    // Determine the opcode based on the operation
    if let Some(dest_reg) = reg {
        if ops.starts_with(&format!("{},(ix", dest_reg)) {
            // LD r,(IX+d)
            let opcode = match dest_reg.as_str() {
                "a" => 0x7e,
                "b" => 0x46,
                "c" => 0x4e,
                "d" => 0x56,
                "e" => 0x5e,
                "h" => 0x66,
                "l" => 0x6e,
                _ => return Ok(None),
            };
            result.push(opcode);
        } else if ops.contains(&format!("),{}", dest_reg)) {
            // LD (IX+d),r
            let opcode = match dest_reg.as_str() {
                "a" => 0x77,
                "b" => 0x70,
                "c" => 0x71,
                "d" => 0x72,
                "e" => 0x73,
                "h" => 0x74,
                "l" => 0x75,
                _ => return Ok(None),
            };
            result.push(opcode);
        }
    }

    result.push(displacement as u8);
    Ok(Some(result))
}

fn handle_iy_indexed_load(ops: &str, constants: &HashMap<String, u16>) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    // Parse the displacement from (IY+d) or (IY-d)
    let (reg, displacement) = parse_indexed_operand(ops, "iy", constants)?;

    result.push(0xfd); // IY prefix

    // Determine the opcode based on the operation
    if let Some(dest_reg) = reg {
        if ops.starts_with(&format!("{},(iy", dest_reg)) {
            // LD r,(IY+d)
            let opcode = match dest_reg.as_str() {
                "a" => 0x7e,
                "b" => 0x46,
                "c" => 0x4e,
                "d" => 0x56,
                "e" => 0x5e,
                "h" => 0x66,
                "l" => 0x6e,
                _ => return Ok(None),
            };
            result.push(opcode);
        } else if ops.contains(&format!("),{}", dest_reg)) {
            // LD (IY+d),r
            let opcode = match dest_reg.as_str() {
                "a" => 0x77,
                "b" => 0x70,
                "c" => 0x71,
                "d" => 0x72,
                "e" => 0x73,
                "h" => 0x74,
                "l" => 0x75,
                _ => return Ok(None),
            };
            result.push(opcode);
        }
    }

    result.push(displacement as u8);
    Ok(Some(result))
}

fn parse_indexed_operand(
    ops: &str,
    index_reg: &str,
    constants: &HashMap<String, u16>,
) -> Result<(Option<String>, i8)> {
    // Extract register and displacement from patterns like "a,(ix+5)" or "(iy-3),b"
    let parts: Vec<&str> = ops.split(',').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Invalid indexed operand format"));
    }

    let (reg, indexed) = if parts[0].contains(index_reg) {
        (Some(parts[1].trim().to_string()), parts[0])
    } else {
        (Some(parts[0].trim().to_string()), parts[1])
    };

    // Extract displacement from (IX+d) or (IY+d)
    let pattern = format!("({}+", index_reg);
    let pattern_neg = format!("({}-", index_reg);

    let displacement = if let Some(pos) = indexed.find(&pattern) {
        let start = pos + pattern.len();
        let end = indexed[start..]
            .find(')')
            .ok_or_else(|| anyhow!("Missing closing parenthesis"))?;
        let disp_str = &indexed[start..start + end];
        parse_immediate(disp_str, constants)? as i8
    } else if let Some(pos) = indexed.find(&pattern_neg) {
        let start = pos + pattern_neg.len();
        let end = indexed[start..]
            .find(')')
            .ok_or_else(|| anyhow!("Missing closing parenthesis"))?;
        let disp_str = &indexed[start..start + end];
        -(parse_immediate(disp_str, constants)? as i8)
    } else if indexed.contains(&format!("({})", index_reg)) {
        0i8
    } else {
        return Err(anyhow!("Invalid indexed addressing format"));
    };

    Ok((reg, displacement))
}
