use anyhow::{anyhow, Result};

/// Handle CB prefix bit manipulation instructions
pub fn handle_bit_instruction(mnemonic: &str, operands: Option<&str>) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    if let Some(ops) = operands {
        let parts: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();

        match mnemonic {
            "bit" | "res" | "set" => {
                if parts.len() != 2 {
                    return Err(anyhow!("{} requires bit number and register", mnemonic));
                }

                let bit_num = parts[0]
                    .parse::<u8>()
                    .map_err(|_| anyhow!("Invalid bit number: {}", parts[0]))?;
                if bit_num > 7 {
                    return Err(anyhow!("Bit number must be 0-7"));
                }

                let reg_code = get_register_code(parts[1])?;

                result.push(0xcb); // CB prefix

                let opcode = match mnemonic {
                    "bit" => 0x40 + (bit_num << 3) + reg_code,
                    "res" => 0x80 + (bit_num << 3) + reg_code,
                    "set" => 0xc0 + (bit_num << 3) + reg_code,
                    _ => unreachable!(),
                };
                result.push(opcode);

                return Ok(Some(result));
            },

            "rlc" | "rrc" | "rl" | "rr" | "sla" | "sra" | "srl" | "sll" => {
                if parts.len() != 1 {
                    return Err(anyhow!("{} requires one register operand", mnemonic));
                }

                let reg_code = get_register_code(parts[0])?;

                result.push(0xcb); // CB prefix

                let opcode = match mnemonic {
                    "rlc" => 0x00 + reg_code,
                    "rrc" => 0x08 + reg_code,
                    "rl" => 0x10 + reg_code,
                    "rr" => 0x18 + reg_code,
                    "sla" => 0x20 + reg_code,
                    "sra" => 0x28 + reg_code,
                    "sll" => 0x30 + reg_code, // Undocumented
                    "srl" => 0x38 + reg_code,
                    _ => unreachable!(),
                };
                result.push(opcode);

                return Ok(Some(result));
            },

            _ => {},
        }
    }

    Ok(None)
}

/// Get CB instruction register code (0-7)
fn get_register_code(reg: &str) -> Result<u8> {
    match reg {
        "b" => Ok(0),
        "c" => Ok(1),
        "d" => Ok(2),
        "e" => Ok(3),
        "h" => Ok(4),
        "l" => Ok(5),
        "(hl)" => Ok(6),
        "a" => Ok(7),
        _ => Err(anyhow!("Invalid register for CB instruction: {}", reg)),
    }
}
