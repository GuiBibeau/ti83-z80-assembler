#[derive(Debug, Clone, PartialEq)]
pub struct ParsedLine {
    pub label: Option<String>,
    pub mnemonic: Option<String>,
    pub operands: Option<String>,
}

pub struct Parser;

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse_line(&self, line: &str) -> Option<ParsedLine> {
        // Remove comments
        let line = if let Some(comment_pos) = line.find(';') {
            &line[..comment_pos]
        } else {
            line
        };

        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let mut label = None;
        let mut remaining = line;

        // Check for label
        if let Some(colon_pos) = line.find(':') {
            label = Some(line[..colon_pos].trim().to_string());
            remaining = line[colon_pos + 1..].trim();
        }

        if remaining.is_empty() {
            return Some(ParsedLine {
                label,
                mnemonic: None,
                operands: None,
            });
        }

        // Parse mnemonic and operands
        let (mnemonic, operands) = if remaining.to_lowercase().starts_with("bcall(") {
            // Special handling for bcall
            let start_idx = remaining.find('(').unwrap();
            let end_idx = remaining.rfind(')').unwrap_or(remaining.len());
            (
                "bcall".to_string(),
                Some(remaining[start_idx + 1..end_idx].trim().to_string()),
            )
        } else {
            // Regular instruction
            if let Some(space_pos) = remaining.find(|c: char| c.is_whitespace()) {
                (
                    remaining[..space_pos].to_lowercase(),
                    Some(remaining[space_pos..].trim().to_string()),
                )
            } else {
                (remaining.to_lowercase(), None)
            }
        };

        Some(ParsedLine {
            label,
            mnemonic: Some(mnemonic),
            operands,
        })
    }

    pub fn split_operands(&self, operands: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0;

        for ch in operands.chars() {
            match ch {
                '(' => {
                    paren_depth += 1;
                    current.push(ch);
                },
                ')' => {
                    paren_depth -= 1;
                    current.push(ch);
                },
                ',' if paren_depth == 0 => {
                    parts.push(current.trim().to_string());
                    current.clear();
                },
                _ => current.push(ch),
            }
        }

        if !current.is_empty() {
            parts.push(current.trim().to_string());
        }

        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_instruction() {
        let parser = Parser::new();
        let result = parser.parse_line("    ld hl, 1234").unwrap();
        assert_eq!(result.label, None);
        assert_eq!(result.mnemonic, Some("ld".to_string()));
        assert_eq!(result.operands, Some("hl, 1234".to_string()));
    }

    #[test]
    fn test_parse_label_and_instruction() {
        let parser = Parser::new();
        let result = parser.parse_line("loop:   inc a").unwrap();
        assert_eq!(result.label, Some("loop".to_string()));
        assert_eq!(result.mnemonic, Some("inc".to_string()));
        assert_eq!(result.operands, Some("a".to_string()));
    }

    #[test]
    fn test_parse_bcall() {
        let parser = Parser::new();
        let result = parser.parse_line("    bcall(_ClrLCDFull)").unwrap();
        assert_eq!(result.mnemonic, Some("bcall".to_string()));
        assert_eq!(result.operands, Some("_ClrLCDFull".to_string()));
    }

    #[test]
    fn test_split_operands() {
        let parser = Parser::new();
        assert_eq!(
            parser.split_operands("hl, ($8000)"),
            vec!["hl".to_string(), "($8000)".to_string()]
        );
        assert_eq!(
            parser.split_operands("a, b"),
            vec!["a".to_string(), "b".to_string()]
        );
    }
}
