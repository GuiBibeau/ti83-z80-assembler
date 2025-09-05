/**
 * CALL and RET instruction handlers
 */

import { parseImmediate } from '../utils/parseImmediate.js';

export function handleCallInstruction(mnemonic, operands, context) {
  const result = [];
  const { labels, constants } = context;

  // CALL instruction (with conditional support)
  if (mnemonic === "call") {
    const parts = operands.split(",").map(p => p.trim());
    if (parts.length === 2) {
      // Conditional call
      const conditionCodes = {
        "z": 0xcc, "nz": 0xc4, "c": 0xdc, "nc": 0xd4,
        "pe": 0xec, "po": 0xe4, "p": 0xf4, "m": 0xfc
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
      // Unconditional call
      const address =
        labels[operands] !== undefined
          ? labels[operands]
          : parseImmediate(operands, constants);
      result.push(0xcd); // CALL nn
      result.push(address & 0xff);
      result.push((address >> 8) & 0xff);
      return new Uint8Array(result);
    }
  }

  return null;
}