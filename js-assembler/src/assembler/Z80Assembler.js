/**
 * Z80 Assembler for TI-83 Plus
 * Main assembler class that orchestrates the assembly process
 */

import { Parser } from './Parser.js';
import { opcodes } from '../instructions/opcodes.js';
import { romCalls } from '../ti83plus/romCalls.js';
import { sysVars } from '../ti83plus/sysVars.js';
import { parseImmediate } from '../utils/parseImmediate.js';
import { handleLoadInstruction } from '../instructions/loads.js';
import { handleJumpInstruction } from '../instructions/jumps.js';
import { handleCallInstruction } from '../instructions/calls.js';
import { handleArithmeticInstruction } from '../instructions/arithmetic.js';
import { handleDataDirective, estimateDataSize } from '../directives/data.js';

export class Z80Assembler {
  constructor() {
    this.parser = new Parser();
    this.instructions = opcodes;
    this.romCalls = romCalls;
    this.sysVars = sysVars;
    
    this.labels = {};
    this.constants = {}; // For .equ directive
    this.output = [];
    this.orgAddress = 0x9d93; // TI-83 Plus programs load at $9D93
    this.currentAddress = this.orgAddress;
  }

  parseImmediate(value) {
    return parseImmediate(value, this.constants);
  }

  assembleInstruction(mnemonic, operands) {
    const result = [];

    // Create context object for handlers
    const context = {
      labels: this.labels,
      constants: this.constants,
      sysVars: this.sysVars,
      romCalls: this.romCalls,
      currentAddress: this.currentAddress,
      splitOperands: (ops) => this.parser.splitOperands(ops),
      parseImmediate: (val) => this.parseImmediate(val)
    };

    // Handle directives
    if (mnemonic === ".org") {
      this.orgAddress = this.parseImmediate(operands);
      this.currentAddress = this.orgAddress;
      return new Uint8Array(0);
    }

    if (mnemonic === ".end") {
      return new Uint8Array(0);
    }
    
    // Handle .equ directive
    if (mnemonic === ".equ") {
      const parts = operands.split(",").map(p => p.trim());
      if (parts.length !== 2) {
        throw new Error(".equ requires name and value");
      }
      const name = parts[0];
      const value = this.parseImmediate(parts[1]);
      this.constants[name] = value;
      return new Uint8Array(0);
    }

    // Handle data directives
    const dataResult = handleDataDirective(mnemonic, operands, context);
    if (dataResult) return dataResult;

    // Handle bcall (ROM calls)
    if (mnemonic === "bcall") {
      let callName = operands;
      if (callName.startsWith("(") && callName.endsWith(")")) {
        callName = callName.slice(1, -1);
      }
      if (this.romCalls[callName] !== undefined) {
        const address = this.romCalls[callName];
        result.push(0xef); // RST 28h (bcall)
        result.push(address & 0xff);
        result.push((address >> 8) & 0xff);
        return new Uint8Array(result);
      } else {
        throw new Error(`Unknown ROM call: ${callName}`);
      }
    }

    // Handle LD instructions
    if (mnemonic === "ld") {
      const ldResult = handleLoadInstruction(operands, context);
      if (ldResult) return ldResult;
    }

    // Handle jump instructions
    const jumpResult = handleJumpInstruction(mnemonic, operands, context);
    if (jumpResult) return jumpResult;

    // Handle call instructions
    const callResult = handleCallInstruction(mnemonic, operands, context);
    if (callResult) return callResult;
    
    // Handle arithmetic instructions
    const arithResult = handleArithmeticInstruction(mnemonic, operands, context);
    if (arithResult) return arithResult;

    // Handle special multi-byte instructions
    if (mnemonic === "reti") {
      result.push(0xed, 0x4d);
      return new Uint8Array(result);
    }
    
    if (mnemonic === "retn") {
      result.push(0xed, 0x45);
      return new Uint8Array(result);
    }

    // Check instruction table
    const fullInst = `${mnemonic} ${operands}`.replace(/\s+/g, " ").trim();
    if (this.instructions[fullInst] !== undefined) {
      const opcode = this.instructions[fullInst];
      if (opcode > 0xffff) {
        result.push((opcode >> 16) & 0xff);
        result.push((opcode >> 8) & 0xff);
        result.push(opcode & 0xff);
      } else if (opcode > 0xff) {
        result.push((opcode >> 8) & 0xff);
        result.push(opcode & 0xff);
      } else {
        result.push(opcode);
      }
      return new Uint8Array(result);
    }
    
    // Check simple mnemonic
    if (this.instructions[mnemonic] !== undefined) {
      const opcode = this.instructions[mnemonic];
      if (opcode > 0xff) {
        result.push((opcode >> 8) & 0xff);
        result.push(opcode & 0xff);
      } else {
        result.push(opcode);
      }
      return new Uint8Array(result);
    }

    // Unknown instruction - throw error instead of silently failing
    if (mnemonic && !mnemonic.startsWith(".")) {
      throw new Error(`Unknown instruction: ${mnemonic} ${operands}`);
    }
    
    return new Uint8Array(0);
  }

  estimateInstructionSize(mnemonic, operands) {
    if ([".org", ".end", ".equ"].includes(mnemonic)) {
      return 0;
    }
    
    // Use data directive size estimator
    if ([".db", ".dw"].includes(mnemonic)) {
      return estimateDataSize(mnemonic, operands);
    }

    if (["bcall", "jp", "call"].includes(mnemonic)) {
      return 3;
    }
    if (mnemonic === "ld") {
      // Check for 16-bit loads
      if (operands.startsWith("hl,") || operands.startsWith("bc,") || 
          operands.startsWith("de,") || operands.startsWith("sp,") ||
          operands.includes("),hl") || operands.includes("),a")) {
        return 3;
      }
      // Check for 8-bit immediate loads
      const parts = operands.split(",");
      if (parts.length === 2 && !parts[1].includes("(")) {
        const reg = parts[0].trim();
        if (["a", "b", "c", "d", "e", "h", "l"].includes(reg)) {
          return 2; // LD r,n
        }
      }
    }
    if (mnemonic === "jr" || mnemonic === "djnz") {
      return 2;
    }
    // Special multi-byte instructions
    if (["reti", "retn"].includes(mnemonic)) {
      return 2;
    }
    // Arithmetic with immediate
    if (["add", "adc", "sub", "sbc", "and", "xor", "or", "cp"].includes(mnemonic)) {
      const parts = operands.split(",");
      if (parts.length === 1 || (parts.length === 2 && parts[0] === "a")) {
        return 2;
      }
    }
    // Default estimate
    return 1;
  }

  assemble(source) {
    const lines = source.split("\n");

    // First pass: collect labels
    this.currentAddress = this.orgAddress;
    for (const line of lines) {
      const parsed = this.parser.parseLine(line);
      if (!parsed) continue;

      const { label, mnemonic, operands } = parsed;

      if (label) {
        this.labels[label] = this.currentAddress;
      }

      if (mnemonic) {
        // Special handling for .org and .equ which don't advance address
        if (mnemonic === ".org") {
          this.orgAddress = this.parseImmediate(operands);
          this.currentAddress = this.orgAddress;
        } else if (mnemonic !== ".equ") {
          this.currentAddress += this.estimateInstructionSize(mnemonic, operands);
        }
      }
    }

    // Second pass: generate code
    this.output = [];
    this.currentAddress = this.orgAddress;

    for (const line of lines) {
      const parsed = this.parser.parseLine(line);
      if (!parsed) continue;

      const { mnemonic, operands } = parsed;

      if (mnemonic) {
        const code = this.assembleInstruction(mnemonic, operands);
        this.output.push(code);
        this.currentAddress += code.length;
      }
    }

    // Combine all output into single Uint8Array
    const totalLength = this.output.reduce((sum, arr) => sum + arr.length, 0);
    const result = new Uint8Array(totalLength);
    let offset = 0;
    for (const arr of this.output) {
      result.set(arr, offset);
      offset += arr.length;
    }

    return result;
  }
}

export default Z80Assembler;