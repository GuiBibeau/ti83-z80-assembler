use crate::utils::immediate::parse_immediate;
use anyhow::Result;
use std::collections::HashMap;

pub fn handle_arithmetic_instruction(
    mnemonic: &str,
    operands: Option<&str>,
    constants: &HashMap<String, u16>,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    // Arithmetic operations with immediate values
    let arith_ops = [
        ("add", 0xc6),
        ("adc", 0xce),
        ("sub", 0xd6),
        ("sbc", 0xde),
        ("and", 0xe6),
        ("xor", 0xee),
        ("or", 0xf6),
        ("cp", 0xfe),
    ];

    for (op_name, opcode) in arith_ops {
        if mnemonic == op_name {
            if let Some(ops) = operands {
                // Check if it's an immediate value (not a register)
                let value_str = if ops.starts_with("a,") {
                    // Format: "add a, n"
                    ops[2..].trim()
                } else {
                    // Format: "add n"
                    ops.trim()
                };

                // Check if it's not a register or memory reference
                if !is_register(value_str) && !value_str.starts_with('(') {
                    result.push(opcode);
                    let value = parse_immediate(value_str, constants)?;
                    result.push((value & 0xff) as u8);
                    return Ok(Some(result));
                }
            }
        }
    }

    // INC/DEC with 8-bit registers handled by opcode table
    // But INC/DEC (IX+d) and (IY+d) would be handled here if needed

    Ok(None)
}

fn is_register(s: &str) -> bool {
    matches!(
        s,
        "a" | "b" | "c" | "d" | "e" | "h" | "l" | "bc" | "de" | "hl" | "sp"
    )
}
