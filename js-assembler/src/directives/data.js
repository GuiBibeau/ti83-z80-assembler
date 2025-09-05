/**
 * Data directive handlers (.db, .dw)
 */

import { parseImmediate } from '../utils/parseImmediate.js';

export function handleDataDirective(mnemonic, operands, context) {
  const result = [];
  const { labels, constants } = context;

  if (mnemonic === ".db") {
    // Data bytes or string - handle mixed string and byte values
    const values = [];
    let current = "";
    let inString = false;
    
    for (let i = 0; i < operands.length; i++) {
      const char = operands[i];
      if (char === '"') {
        inString = !inString;
        current += char;
      } else if (char === ',' && !inString) {
        values.push(current.trim());
        current = "";
      } else {
        current += char;
      }
    }
    if (current) {
      values.push(current.trim());
    }

    for (const value of values) {
      if (value.startsWith('"') && value.endsWith('"')) {
        // String value
        const text = value
          .slice(1, -1)
          .replace(/\\n/g, "\n")
          .replace(/\\r/g, "\r")
          .replace(/\\t/g, "\t");
        for (let i = 0; i < text.length; i++) {
          result.push(text.charCodeAt(i));
        }
      } else if (labels[value] !== undefined) {
        result.push(labels[value] & 0xff);
      } else {
        result.push(parseImmediate(value, constants) & 0xff);
      }
    }
    return new Uint8Array(result);
  }

  if (mnemonic === ".dw") {
    // Data words (16-bit, little-endian)
    const values = operands.split(",");
    for (const value of values) {
      const trimmed = value.trim();
      let word;
      if (labels[trimmed] !== undefined) {
        word = labels[trimmed];
      } else {
        word = parseImmediate(trimmed, constants);
      }
      result.push(word & 0xff);
      result.push((word >> 8) & 0xff);
    }
    return new Uint8Array(result);
  }

  return null;
}

export function estimateDataSize(mnemonic, operands) {
  if (mnemonic === ".db") {
    // Count bytes for mixed string and byte values
    let count = 0;
    const values = [];
    let current = "";
    let inString = false;
    
    for (let i = 0; i < operands.length; i++) {
      const char = operands[i];
      if (char === '"') {
        inString = !inString;
        current += char;
      } else if (char === ',' && !inString) {
        values.push(current.trim());
        current = "";
      } else {
        current += char;
      }
    }
    if (current) {
      values.push(current.trim());
    }

    for (const value of values) {
      if (value.startsWith('"') && value.endsWith('"')) {
        // String value - count characters
        const text = value.slice(1, -1)
          .replace(/\\n/g, "\n")
          .replace(/\\r/g, "\r")
          .replace(/\\t/g, "\t");
        count += text.length;
      } else {
        // Single byte
        count += 1;
      }
    }
    return count;
  }

  if (mnemonic === ".dw") {
    return operands.split(",").length * 2;
  }

  return 0;
}