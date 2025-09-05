/**
 * Parse immediate values from assembly source
 * Handles hex ($FF, 0xFF), binary (%10101010), decimal, and character literals
 */

export function parseImmediate(value, constants = {}) {
  value = value.trim();
  
  // Check if it's a constant
  if (constants[value] !== undefined) {
    return constants[value];
  }

  // Handle character literals
  if (value.startsWith("'") && value.endsWith("'") && value.length === 3) {
    return value.charCodeAt(1);
  }

  // Hex values
  if (value.startsWith("$")) {
    return parseInt(value.substring(1), 16);
  } else if (value.startsWith("0x")) {
    return parseInt(value.substring(2), 16);
  }
  // Binary values
  else if (value.startsWith("%")) {
    return parseInt(value.substring(1), 2);
  }
  // Decimal
  else {
    return parseInt(value, 10);
  }
}

export default parseImmediate;