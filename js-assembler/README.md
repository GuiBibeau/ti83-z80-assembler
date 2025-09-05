# JavaScript Z80 Assembler

**WORK IN PROGRESS** - This assembler is incomplete and supports only a subset of Z80 instructions. For a complete implementation, use the Rust assembler in `../rust-assembler/`.

Fast, lightweight Z80 assembler for TI-83 Plus calculator programs.

## Status

- [x] Basic instructions (LD, JP, CALL, RET)
- [x] ROM calls (bcall)
- [x] Data directives (.db, .dw)
- [ ] Bit manipulation instructions (incomplete)
- [ ] Index register operations (partial)
- [ ] Extended instructions (limited)
- [ ] Full instruction set coverage (~40% implemented)

## Installation

```bash
# Using Bun (recommended)
bun install

# Using npm
npm install
```

## Usage

```bash
# Basic usage
bun z80asm.js input.asm output.8xp

# Or with node
node z80asm.js input.asm output.8xp
```

## Features

- Zero dependencies
- Runs in browser, Node.js, or Bun
- Modular architecture
- Fast assembly times

## Project Structure

```
js-assembler/
├── z80asm.js          # CLI entry point
└── src/
    ├── index.js       # Main module exports
    ├── assembler/     # Core assembly logic
    ├── instructions/  # Instruction handlers
    ├── directives/    # Directive processors
    ├── ti83plus/      # TI-83 specific code
    └── utils/         # Utilities
```

## Testing

```bash
# Assemble example programs
bun z80asm.js ../examples/hello.asm hello.8xp
bun z80asm.js ../examples/math.asm math.8xp
```

## Limitations

- Incomplete instruction set support
- Limited testing
- May produce incorrect output for complex programs
- Use for simple programs only or as a reference implementation

## Recommended Alternative

For production use, please use the Rust assembler in `../rust-assembler/` which provides:
- Complete Z80 instruction set
- Comprehensive testing
- Better performance
- Active development