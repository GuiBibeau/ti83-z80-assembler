/**
 * LD instruction handlers
 */

import { parseImmediate } from '../utils/parseImmediate.js';

export function handleLoadInstruction(operands, context) {
  const result = [];
  const { labels, constants, sysVars, currentAddress, splitOperands } = context;
  
  // Split operands carefully to handle parentheses
  const parts = splitOperands(operands);

  // 8-bit register load codes
  const regCodes = {
    b: 0x06,
    c: 0x0e,
    d: 0x16,
    e: 0x1e,
    h: 0x26,
    l: 0x2e,
    a: 0x3e,
  };

  // LD r,n (8-bit immediate)
  if (regCodes[parts[0]] && !parts[1].startsWith("(")) {
    result.push(regCodes[parts[0]]);
    const value =
      labels[parts[1]] !== undefined
        ? labels[parts[1]]
        : parseImmediate(parts[1], constants);
    result.push(value & 0xff);
    return new Uint8Array(result);
  }

  // LD HL,(nn) - check this before LD HL,nn
  if (parts[0] === "hl" && parts[1].startsWith("(") && parts[1].endsWith(")")) {
    const addrStr = parts[1].slice(1, -1);
    const address =
      sysVars[addrStr] !== undefined
        ? sysVars[addrStr]
        : parseImmediate(addrStr, constants);
    result.push(0x2a); // LD HL,(nn)
    result.push(address & 0xff);
    result.push((address >> 8) & 0xff);
    return new Uint8Array(result);
  }
  
  // LD HL,nn (16-bit immediate)
  if (parts[0] === "hl") {
    let value;
    if (labels[parts[1]] !== undefined) {
      value = labels[parts[1]];
    } else if (sysVars[parts[1]] !== undefined) {
      value = sysVars[parts[1]];
    } else {
      value = parseImmediate(parts[1], constants);
    }
    result.push(0x21); // LD HL,nn
    result.push(value & 0xff);
    result.push((value >> 8) & 0xff);
    return new Uint8Array(result);
  }

  // LD BC,nn
  if (parts[0] === "bc") {
    const value =
      labels[parts[1]] !== undefined
        ? labels[parts[1]]
        : parseImmediate(parts[1], constants);
    result.push(0x01);
    result.push(value & 0xff);
    result.push((value >> 8) & 0xff);
    return new Uint8Array(result);
  }

  // LD DE,nn
  if (parts[0] === "de") {
    const value =
      labels[parts[1]] !== undefined
        ? labels[parts[1]]
        : parseImmediate(parts[1], constants);
    result.push(0x11);
    result.push(value & 0xff);
    result.push((value >> 8) & 0xff);
    return new Uint8Array(result);
  }

  // LD SP,nn
  if (parts[0] === "sp") {
    const value =
      labels[parts[1]] !== undefined
        ? labels[parts[1]]
        : parseImmediate(parts[1], constants);
    result.push(0x31);
    result.push(value & 0xff);
    result.push((value >> 8) & 0xff);
    return new Uint8Array(result);
  }

  // LD (nn),HL
  if (
    parts[0].startsWith("(") &&
    parts[0].endsWith(")") &&
    parts[1] === "hl"
  ) {
    const addrStr = parts[0].slice(1, -1);
    const address =
      sysVars[addrStr] !== undefined
        ? sysVars[addrStr]
        : parseImmediate(addrStr, constants);
    result.push(0x22); // LD (nn),HL
    result.push(address & 0xff);
    result.push((address >> 8) & 0xff);
    return new Uint8Array(result);
  }
  
  // LD A,(nn)
  if (parts[0] === "a" && parts[1].startsWith("(") && parts[1].endsWith(")")) {
    const inner = parts[1].slice(1, -1);
    // Check if it's a register pair first
    if (!["bc", "de", "hl"].includes(inner)) {
      const address =
        sysVars[inner] !== undefined
          ? sysVars[inner]
          : parseImmediate(inner, constants);
      result.push(0x3a); // LD A,(nn)
      result.push(address & 0xff);
      result.push((address >> 8) & 0xff);
      return new Uint8Array(result);
    }
  }
  
  // LD (nn),A
  if (parts[0].startsWith("(") && parts[0].endsWith(")") && parts[1] === "a") {
    const addrStr = parts[0].slice(1, -1);
    // Check if it's a register pair first
    if (!["bc", "de", "hl"].includes(addrStr)) {
      const address =
        sysVars[addrStr] !== undefined
          ? sysVars[addrStr]
          : parseImmediate(addrStr, constants);
      result.push(0x32); // LD (nn),A
      result.push(address & 0xff);
      result.push((address >> 8) & 0xff);
      return new Uint8Array(result);
    }
  }

  return null; // Not handled here
}