/**
 * Jump instruction handlers (JP, JR, DJNZ)
 */

import { parseImmediate } from '../utils/parseImmediate.js';

export function handleJumpInstruction(mnemonic, operands, context) {
  const result = [];
  const { labels, constants, currentAddress } = context;

  // JP instruction (with conditional support)
  if (mnemonic === "jp") {
    const parts = operands.split(",").map(p => p.trim());
    if (parts.length === 2) {
      // Conditional jump
      const conditionCodes = {
        "z": 0xca, "nz": 0xc2, "c": 0xda, "nc": 0xd2,
        "pe": 0xea, "po": 0xe2, "p": 0xf2, "m": 0xfa
      };
      if (conditionCodes[parts[0]]) {
        const address = labels[parts[1]] !== undefined
          ? labels[parts[1]]
          : parseImmediate(parts[1], constants);
        result.push(conditionCodes[parts[0]]);
        result.push(address & 0xff);
        result.push((address >> 8) & 0xff);
        return new Uint8Array(result);
      }
    } else {
      // Unconditional jump
      const address =
        labels[operands] !== undefined
          ? labels[operands]
          : parseImmediate(operands, constants);
      result.push(0xc3); // JP nn
      result.push(address & 0xff);
      result.push((address >> 8) & 0xff);
      return new Uint8Array(result);
    }
  }

  // JR instruction (relative jump with conditional support)
  if (mnemonic === "jr") {
    const parts = operands.split(",").map(p => p.trim());
    let opcode = 0x18; // Default: unconditional JR
    let target;
    
    if (parts.length === 2) {
      // Conditional JR
      const conditions = {
        "z": 0x28, "nz": 0x20, "c": 0x38, "nc": 0x30
      };
      if (conditions[parts[0]]) {
        opcode = conditions[parts[0]];
        target = parts[1];
      } else {
        throw new Error(`Invalid JR condition: ${parts[0]}`);
      }
    } else {
      // Unconditional JR
      target = operands;
    }
    
    // Calculate relative offset
    let offset;
    if (labels[target] !== undefined) {
      offset = labels[target] - (currentAddress + 2);
    } else {
      // Try to parse as immediate offset
      offset = parseImmediate(target, constants);
    }
    
    if (offset < -128 || offset > 127) {
      throw new Error(`JR offset too large: ${offset}`);
    }
    
    result.push(opcode);
    result.push(offset & 0xff);
    return new Uint8Array(result);
  }
  
  // DJNZ instruction (decrement B and jump if not zero)
  if (mnemonic === "djnz") {
    let offset;
    if (labels[operands] !== undefined) {
      offset = labels[operands] - (currentAddress + 2);
    } else {
      offset = parseImmediate(operands, constants);
    }
    
    if (offset < -128 || offset > 127) {
      throw new Error(`DJNZ offset too large: ${offset}`);
    }
    
    result.push(0x10); // DJNZ e
    result.push(offset & 0xff);
    return new Uint8Array(result);
  }

  return null;
}