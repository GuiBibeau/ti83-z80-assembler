/**
 * Arithmetic and logic instruction handlers
 */

import { parseImmediate } from '../utils/parseImmediate.js';

export function handleArithmeticInstruction(mnemonic, operands, context) {
  const result = [];
  const { labels, constants } = context;

  // Arithmetic/Logic with immediate
  const arithmeticOps = {
    "add": 0xc6, "adc": 0xce, "sub": 0xd6, "sbc": 0xde,
    "and": 0xe6, "xor": 0xee, "or": 0xf6, "cp": 0xfe
  };
  
  if (arithmeticOps[mnemonic]) {
    const parts = operands.split(",").map(p => p.trim());
    // Handle both "add a,n" and "sub n" formats
    let value;
    if (parts.length === 2 && parts[0] === "a") {
      value = parts[1];
    } else if (parts.length === 1) {
      value = parts[0];
    }
    
    if (value !== undefined) {
      result.push(arithmeticOps[mnemonic]);
      const immediate = labels[value] !== undefined
        ? labels[value]
        : parseImmediate(value, constants);
      result.push(immediate & 0xff);
      return new Uint8Array(result);
    }
  }

  return null;
}