use crate::assembler::parser::Parser;
use crate::instructions::opcodes::REG_LOAD_IMMEDIATE;
use crate::ti83plus::sys_vars::SYS_VARS;
use crate::utils::immediate::parse_immediate;
use anyhow::Result;
use std::collections::HashMap;

pub fn handle_load_instruction(
    operands: &str,
    labels: &HashMap<String, u16>,
    constants: &HashMap<String, u16>,
    parser: &Parser,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();
    let parts = parser.split_operands(operands);

    if parts.len() != 2 {
        return Ok(None);
    }

    let dest = &parts[0];
    let src = &parts[1];

    // LD r,n (8-bit immediate) - but only if source is not a register
    if let Some(&opcode) = REG_LOAD_IMMEDIATE.get(dest.as_str()) {
        if !src.starts_with('(') && !is_register(src) {
            result.push(opcode);
            let value = if let Some(&label_addr) = labels.get(src.as_str()) {
                label_addr
            } else {
                parse_immediate(src, constants)?
            };
            result.push((value & 0xff) as u8);
            return Ok(Some(result));
        }
    }

    // LD HL,(nn)
    if dest == "hl" && src.starts_with('(') && src.ends_with(')') {
        let addr_str = &src[1..src.len() - 1];
        let address = if let Some(&sys_var) = SYS_VARS.get(addr_str) {
            sys_var
        } else {
            parse_immediate(addr_str, constants)?
        };
        result.push(0x2a); // LD HL,(nn)
        result.push((address & 0xff) as u8);
        result.push(((address >> 8) & 0xff) as u8);
        return Ok(Some(result));
    }

    // LD HL,nn (16-bit immediate)
    if dest == "hl" {
        let value = if let Some(&label_addr) = labels.get(src.as_str()) {
            label_addr
        } else if let Some(&sys_var) = SYS_VARS.get(src.as_str()) {
            sys_var
        } else {
            parse_immediate(src, constants)?
        };
        result.push(0x21); // LD HL,nn
        result.push((value & 0xff) as u8);
        result.push(((value >> 8) & 0xff) as u8);
        return Ok(Some(result));
    }

    // LD BC,nn
    if dest == "bc" {
        let value = if let Some(&label_addr) = labels.get(src.as_str()) {
            label_addr
        } else {
            parse_immediate(src, constants)?
        };
        result.push(0x01);
        result.push((value & 0xff) as u8);
        result.push(((value >> 8) & 0xff) as u8);
        return Ok(Some(result));
    }

    // LD DE,nn
    if dest == "de" {
        let value = if let Some(&label_addr) = labels.get(src.as_str()) {
            label_addr
        } else {
            parse_immediate(src, constants)?
        };
        result.push(0x11);
        result.push((value & 0xff) as u8);
        result.push(((value >> 8) & 0xff) as u8);
        return Ok(Some(result));
    }

    // LD SP,nn
    if dest == "sp" {
        let value = if let Some(&label_addr) = labels.get(src.as_str()) {
            label_addr
        } else {
            parse_immediate(src, constants)?
        };
        result.push(0x31);
        result.push((value & 0xff) as u8);
        result.push(((value >> 8) & 0xff) as u8);
        return Ok(Some(result));
    }

    // LD (nn),HL
    if dest.starts_with('(') && dest.ends_with(')') && src == "hl" {
        let addr_str = &dest[1..dest.len() - 1];
        let address = if let Some(&sys_var) = SYS_VARS.get(addr_str) {
            sys_var
        } else {
            parse_immediate(addr_str, constants)?
        };
        result.push(0x22); // LD (nn),HL
        result.push((address & 0xff) as u8);
        result.push(((address >> 8) & 0xff) as u8);
        return Ok(Some(result));
    }

    // LD A,(nn)
    if dest == "a" && src.starts_with('(') && src.ends_with(')') {
        let inner = &src[1..src.len() - 1];
        // Check if it's a register pair first
        if !["bc", "de", "hl"].contains(&inner) {
            let address = if let Some(&sys_var) = SYS_VARS.get(inner) {
                sys_var
            } else {
                parse_immediate(inner, constants)?
            };
            result.push(0x3a); // LD A,(nn)
            result.push((address & 0xff) as u8);
            result.push(((address >> 8) & 0xff) as u8);
            return Ok(Some(result));
        }
    }

    // LD (nn),A
    if dest.starts_with('(') && dest.ends_with(')') && src == "a" {
        let addr_str = &dest[1..dest.len() - 1];
        // Check if it's a register pair first
        if !["bc", "de", "hl"].contains(&addr_str.as_ref()) {
            let address = if let Some(&sys_var) = SYS_VARS.get(addr_str) {
                sys_var
            } else {
                parse_immediate(addr_str, constants)?
            };
            result.push(0x32); // LD (nn),A
            result.push((address & 0xff) as u8);
            result.push(((address >> 8) & 0xff) as u8);
            return Ok(Some(result));
        }
    }

    Ok(None) // Not handled here
}

fn is_register(s: &str) -> bool {
    matches!(s, "a" | "b" | "c" | "d" | "e" | "h" | "l" | "(hl)")
}
