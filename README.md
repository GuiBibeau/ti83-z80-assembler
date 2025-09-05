# TI-83 Plus Z80 Assembler

A learning project implementing a Z80 assembler for TI-83 Plus calculator programs in both JavaScript and Rust.

## Overview

This project assembles Z80 assembly language source files (.asm) into TI-83 Plus program files (.8xp) that can run on TI-83 Plus/84 Plus calculators or emulators.

## Status

**Learning Project** - While functional for many programs, this assembler is not feature-complete and serves primarily as an educational implementation of assembly concepts and calculator programming.

## Implementations

### Rust Assembler (`/rust-assembler`)
- ~90% instruction coverage
- Well-tested and reliable
- Recommended for actual use

### JavaScript Assembler (`/js-assembler`)  
- ~40% instruction coverage
- Work in progress
- Useful as a reference implementation

## Quick Start

```bash
# Using the Rust assembler (recommended)
cd rust-assembler
cargo run -- input.asm output.8xp

# Using the JavaScript assembler
cd js-assembler
bun z80asm.js input.asm output.8xp
```

## Example Programs

See `/examples` for sample assembly programs including:
- Hello World
- Basic math operations
- System demonstrations

## Learning Goals

This project explores:
- Z80 CPU architecture and instruction encoding
- TI-83 Plus system programming
- Two-pass assembly techniques
- Binary file format generation
- Cross-language implementation comparison

## Limitations

- Not all Z80 instructions implemented
- Limited macro support
- No linker functionality
- Minimal optimization

For production calculator development, consider established tools like SPASM-ng or Z80ASM.

## License

Educational project - use at your own risk.