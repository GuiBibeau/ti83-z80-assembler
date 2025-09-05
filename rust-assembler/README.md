# Z80 Assembler for TI-83 Plus (Rust Implementation)

A high-performance Z80 assembler for TI-83 Plus calculator programs, ported from JavaScript to Rust.

## Features

- Full Z80 instruction set support
- TI-83 Plus specific ROM calls (bcall)
- Assembly directives (.org, .db, .dw, .equ)
- Label and constant support
- Generates valid .8xp files ready for transfer to calculator
- ~10x faster than the JavaScript implementation
- Byte-for-byte compatible output with the original assembler

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Basic usage
z80asm input.asm [output.8xp]

# With custom program name
z80asm input.asm -n MYPROG output.8xp
```

## Example Assembly Program

```asm
; hello.asm - Display "Hello World!" on TI-83 Plus
.org $9D93
.db $BB,$6D
    bcall(_ClrLCDFull)  ; Clear the screen
    bcall(_HomeUp)      ; Move cursor to home position
    ld hl,message       ; Load address of message
    bcall(_PutS)        ; Display the string
    ret                 ; Return to TI-OS

message:
    .db "Hello World!",0 ; Null-terminated string
```

## Supported Features

- **Full Z80 Instruction Set**: All standard Z80 CPU instructions
- **IX/IY Registers**: Index register operations and indexed addressing
- **Bit Manipulation**: BIT, SET, RES, and rotate/shift operations (CB prefix)
- **Block Operations**: LDIR, CPIR, and other block transfer instructions
- **I/O Port Instructions**: IN/OUT for hardware control
- **100+ ROM Calls**: Extensive TI-OS function support including:
  - Display routines (_ClrLCDFull, _PutS, _VPutS, etc.)
  - Math operations (_FPAdd, _FPMult, _Sin, _Cos, etc.)
  - Graphics (_ILine, _IPoint, _DrawCircle, etc.)
  - Variable management (_ChkFindSym, _CreateReal, etc.)

## Testing

```bash
# Run all tests
cargo test

# Run with example files
cargo run -- tests/fixtures/hello.asm hello.8xp
cargo run -- tests/fixtures/math.asm math.8xp
```

## Compatibility

This Rust implementation produces byte-for-byte identical output to the original JavaScript assembler, ensuring complete compatibility with existing assembly programs and the TI-83 Plus calculator.

### Known Issues

**jsTIfied Emulator**: CB prefix instructions (SET, RES, BIT, etc.) cause display corruption in the jsTIfied web emulator. This is an emulator bug, not an assembler issue. Programs work correctly on real hardware. For jsTIfied testing, use these workarounds:

```asm
; Instead of CB instructions:
or $40      ; Instead of SET 6,A
and $FE     ; Instead of RES 0,A  
rlca        ; Instead of RLC A (CB version)
```

**IX/IY Registers**: The TI-83 Plus OS uses IX as a system pointer. Always preserve IX when using it:

```asm
push ix     ; Save IX
; ... use IX ...
pop ix      ; Restore IX
```

**Program Names**: Limited to 8 characters, uppercase alphanumeric only, no underscores.

## Architecture

- **Parser**: Tokenizes assembly source into labels, mnemonics, and operands
- **Assembler**: Two-pass assembly with label resolution
- **Instruction Handlers**: Modular handlers for different instruction types
- **TI8XP Generator**: Creates valid calculator program files with proper headers and checksums

## Performance

The Rust implementation offers significant performance improvements:
- ~10x faster assembly times
- Zero-copy parsing where possible
- Efficient static opcode lookup tables using perfect hash maps
- Memory-safe with no buffer overflows

## License

MIT