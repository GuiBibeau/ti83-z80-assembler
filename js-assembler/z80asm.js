#!/usr/bin/env bun
/**
 * TI-83 Plus Z80 Assembler CLI
 * Command-line interface for the Z80 assembler
 * Run with: bun z80asm.js input.asm [output.8xp]
 */

import { writeFile, readFile } from "fs/promises";
import { basename } from "path";
import { Z80Assembler, TI8XPGenerator } from "./src/index.js";

async function main() {
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.log("Usage: bun z80asm.js input.asm [output.8xp]");
    console.log("       bunx z80asm input.asm [output.8xp]");
    process.exit(1);
  }

  const inputFile = args[0];
  const outputFile = args[1] || inputFile.replace(/\.[^.]+$/, ".8xp");
  const programName = basename(inputFile)
    .replace(/\.[^.]+$/, "")
    .substring(0, 8);

  try {
    // Read source code
    const source = await readFile(inputFile, "utf-8");

    // Create assembler instance
    const assembler = new Z80Assembler();

    // Add TI-83 Plus header if not present
    let processedSource = source;
    if (!source.includes(".org")) {
      processedSource = ".org $9D93\n.db $BB,$6D\n" + source;
    }

    // Assemble the code
    const code = assembler.assemble(processedSource);
    console.log(`✓ Assembled ${code.length} bytes`);

    // Generate .8xp file
    const output = TI8XPGenerator.create8xp(programName, code);

    // Write output file
    await writeFile(outputFile, output);

    console.log(`✓ Created ${outputFile} (${output.length} bytes)`);
    console.log(`✓ Program name: ${programName.toUpperCase()}`);
    console.log("\nTo test:");
    console.log("1. Visit https://www.cemetech.net/projects/jstified/");
    console.log(`2. Drag ${outputFile} onto the calculator`);
    console.log(`3. Run with: Asm(prgm${programName.toUpperCase()})`);
  } catch (error) {
    console.error(`Error: ${error.message}`);
    process.exit(1);
  }
}

// Run if executed directly
if (import.meta.main) {
  main();
}

// Export for use as module
export { Z80Assembler, TI8XPGenerator };