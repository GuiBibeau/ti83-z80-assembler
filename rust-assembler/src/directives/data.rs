use crate::utils::immediate::parse_immediate;
use anyhow::Result;
use std::collections::HashMap;

pub fn handle_data_directive(
    mnemonic: &str,
    operands: Option<&str>,
    labels: &HashMap<String, u16>,
    constants: &HashMap<String, u16>,
) -> Result<Option<Vec<u8>>> {
    let mut result = Vec::new();

    match mnemonic {
        ".db" => {
            if let Some(ops) = operands {
                let values = parse_db_operands(ops);
                for value in values {
                    if value.starts_with('"') && value.ends_with('"') {
                        // String value
                        let text = &value[1..value.len() - 1];
                        let text = text
                            .replace("\\n", "\n")
                            .replace("\\r", "\r")
                            .replace("\\t", "\t");
                        for ch in text.chars() {
                            result.push(ch as u8);
                        }
                    } else if let Some(&label_addr) = labels.get(value.as_str()) {
                        result.push((label_addr & 0xff) as u8);
                    } else {
                        let byte_val = parse_immediate(&value, constants)?;
                        result.push((byte_val & 0xff) as u8);
                    }
                }
                return Ok(Some(result));
            }
        },
        ".dw" => {
            if let Some(ops) = operands {
                let values: Vec<&str> = ops.split(',').map(|s| s.trim()).collect();
                for value in values {
                    let word = if let Some(&label_addr) = labels.get(value) {
                        label_addr
                    } else {
                        parse_immediate(value, constants)?
                    };
                    result.push((word & 0xff) as u8);
                    result.push(((word >> 8) & 0xff) as u8);
                }
                return Ok(Some(result));
            }
        },
        _ => {},
    }

    Ok(None)
}

pub fn estimate_data_size(mnemonic: &str, operands: &str) -> usize {
    match mnemonic {
        ".db" => {
            let values = parse_db_operands(operands);
            let mut count = 0;
            for value in values {
                if value.starts_with('"') && value.ends_with('"') {
                    // String value - count characters
                    let text = &value[1..value.len() - 1];
                    let text = text
                        .replace("\\n", "\n")
                        .replace("\\r", "\r")
                        .replace("\\t", "\t");
                    count += text.len();
                } else {
                    // Single byte
                    count += 1;
                }
            }
            count
        },
        ".dw" => operands.split(',').count() * 2,
        _ => 0,
    }
}

fn parse_db_operands(operands: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_string = false;

    for ch in operands.chars() {
        match ch {
            '"' => {
                in_string = !in_string;
                current.push(ch);
            },
            ',' if !in_string => {
                values.push(current.trim().to_string());
                current.clear();
            },
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        values.push(current.trim().to_string());
    }

    values
}
