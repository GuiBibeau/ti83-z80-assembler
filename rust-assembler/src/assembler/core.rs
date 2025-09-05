use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::assembler::parser::Parser;
use crate::constants::{RST_28H, TI83_PLUS_ORIGIN};
use crate::directives::handle_data_directive;
use crate::instructions::opcodes::{OPCODES, REG_LOAD_IMMEDIATE};
use crate::instructions::{
    handle_arithmetic_instruction, handle_bit_instruction, handle_call_instruction,
    handle_index_instruction, handle_io_instruction, handle_jump_instruction,
    handle_load_instruction,
};
use crate::ti83plus::rom_calls::ROM_CALLS;
use crate::utils::immediate::parse_immediate;

pub struct Z80Assembler {
    parser: Parser,
    labels: HashMap<String, u16>,
    constants: HashMap<String, u16>,
    org_address: u16,
    current_address: u16,
}

impl Default for Z80Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Z80Assembler {
    pub fn new() -> Self {
        Z80Assembler {
            parser: Parser::new(),
            labels: HashMap::new(),
            constants: HashMap::new(),
            org_address: TI83_PLUS_ORIGIN,
            current_address: TI83_PLUS_ORIGIN,
        }
    }

    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>> {
        let lines: Vec<&str> = source.lines().collect();

        self.current_address = self.org_address;
        for line in &lines {
            if let Some(parsed) = self.parser.parse_line(line) {
                if let Some(label) = parsed.label {
                    self.labels.insert(label, self.current_address);
                }

                if let Some(mnemonic) = &parsed.mnemonic {
                    if mnemonic == ".org" {
                        if let Some(operands) = &parsed.operands {
                            self.org_address = parse_immediate(operands, &self.constants)?;
                            self.current_address = self.org_address;
                        }
                    } else if mnemonic != ".equ" {
                        let size =
                            self.estimate_instruction_size(mnemonic, parsed.operands.as_deref());
                        self.current_address = self.current_address.wrapping_add(size as u16);
                    }
                }
            }
        }

        let mut output = Vec::new();
        self.current_address = self.org_address;

        for line in &lines {
            if let Some(parsed) = self.parser.parse_line(line) {
                if let Some(mnemonic) = parsed.mnemonic {
                    let code = self.assemble_instruction(&mnemonic, parsed.operands.as_deref())?;
                    output.extend_from_slice(&code);
                    self.current_address = self.current_address.wrapping_add(code.len() as u16);
                }
            }
        }

        Ok(output)
    }

    fn assemble_instruction(&mut self, mnemonic: &str, operands: Option<&str>) -> Result<Vec<u8>> {
        let mut result = Vec::new();

        match mnemonic {
            ".org" => {
                if let Some(ops) = operands {
                    self.org_address = parse_immediate(ops, &self.constants)?;
                    self.current_address = self.org_address;
                }
                return Ok(vec![]);
            },
            ".end" => return Ok(vec![]),
            ".equ" => {
                if let Some(ops) = operands {
                    let parts: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();
                    if parts.len() != 2 {
                        return Err(anyhow!(".equ requires name and value"));
                    }
                    let value = parse_immediate(parts[1], &self.constants)?;
                    self.constants.insert(parts[0].to_string(), value);
                }
                return Ok(vec![]);
            },
            _ => {},
        }

        if let Some(data) =
            handle_data_directive(mnemonic, operands, &self.labels, &self.constants)?
        {
            return Ok(data);
        }

        if mnemonic == "bcall" {
            if let Some(ops) = operands {
                let call_name = ops.trim_start_matches('(').trim_end_matches(')');
                if let Some(&address) = ROM_CALLS.get(call_name) {
                    result.push(RST_28H);
                    result.push((address & 0xff) as u8);
                    result.push(((address >> 8) & 0xff) as u8);
                    return Ok(result);
                } else {
                    return Err(anyhow!("Unknown ROM call: {}", call_name));
                }
            }
        }

        if let Some(code) =
            handle_index_instruction(mnemonic, operands, &self.labels, &self.constants)?
        {
            return Ok(code);
        }

        if mnemonic == "ld" {
            if let Some(ops) = operands {
                if let Some(code) =
                    handle_load_instruction(ops, &self.labels, &self.constants, &self.parser)?
                {
                    return Ok(code);
                }
            }
        }

        if let Some(code) = handle_jump_instruction(
            mnemonic,
            operands,
            &self.labels,
            &self.constants,
            self.current_address,
        )? {
            return Ok(code);
        }

        if let Some(code) =
            handle_call_instruction(mnemonic, operands, &self.labels, &self.constants)?
        {
            return Ok(code);
        }

        if let Some(code) = handle_arithmetic_instruction(mnemonic, operands, &self.constants)? {
            return Ok(code);
        }

        if let Some(code) = handle_bit_instruction(mnemonic, operands)? {
            return Ok(code);
        }

        if let Some(code) = handle_io_instruction(mnemonic, operands, &self.constants)? {
            return Ok(code);
        }

        if mnemonic == "reti" {
            return Ok(vec![0xed, 0x4d]);
        }
        if mnemonic == "retn" {
            return Ok(vec![0xed, 0x45]);
        }

        let full_inst = if let Some(ops) = operands {
            format!("{} {}", mnemonic, ops)
        } else {
            mnemonic.to_string()
        };

        if let Some(&opcode) = OPCODES.get(full_inst.as_str()) {
            if opcode > 0xffff {
                result.push(((opcode >> 16) & 0xff) as u8);
                result.push(((opcode >> 8) & 0xff) as u8);
                result.push((opcode & 0xff) as u8);
            } else if opcode > 0xff {
                result.push(((opcode >> 8) & 0xff) as u8);
                result.push((opcode & 0xff) as u8);
            } else {
                result.push(opcode as u8);
            }
            return Ok(result);
        }

        if let Some(&opcode) = OPCODES.get(mnemonic) {
            if opcode > 0xff {
                result.push(((opcode >> 8) & 0xff) as u8);
                result.push((opcode & 0xff) as u8);
            } else {
                result.push(opcode as u8);
            }
            return Ok(result);
        }

        Err(anyhow!(
            "Unknown instruction: {} {}",
            mnemonic,
            operands.unwrap_or("")
        ))
    }

    fn estimate_instruction_size(&self, mnemonic: &str, operands: Option<&str>) -> usize {
        match mnemonic {
            ".org" | ".end" | ".equ" => 0,
            ".db" => {
                if let Some(ops) = operands {
                    crate::directives::estimate_data_size(mnemonic, ops)
                } else {
                    0
                }
            },
            ".dw" => {
                if let Some(ops) = operands {
                    ops.split(',').count() * 2
                } else {
                    0
                }
            },
            "bcall" | "jp" | "call" => 3,
            "ld" => {
                if let Some(ops) = operands {
                    if ops.starts_with("hl,")
                        || ops.starts_with("bc,")
                        || ops.starts_with("de,")
                        || ops.starts_with("sp,")
                        || ops.contains("),hl")
                        || ops.contains("),a")
                    {
                        3
                    } else {
                        let parts = self.parser.split_operands(ops);
                        if parts.len() == 2 && !parts[1].contains('(') {
                            if REG_LOAD_IMMEDIATE.contains_key(parts[0].as_str()) {
                                2
                            } else {
                                1
                            }
                        } else {
                            1
                        }
                    }
                } else {
                    1
                }
            },
            "jr" | "djnz" => 2,
            "reti" | "retn" => 2,
            "add" | "adc" | "sub" | "sbc" | "and" | "xor" | "or" | "cp" => {
                if let Some(ops) = operands {
                    let parts = self.parser.split_operands(ops);
                    if parts.len() == 1 || (parts.len() == 2 && parts[0] == "a") {
                        if !parts.last().unwrap().starts_with('(') {
                            2
                        } else {
                            1
                        }
                    } else {
                        1
                    }
                } else {
                    1
                }
            },
            _ => 1,
        }
    }
}
