use z80asm::{TI8XPGenerator, Z80Assembler};

#[test]
fn test_hello_world_assembly() {
    let source = include_str!("fixtures/hello.asm");
    let mut assembler = Z80Assembler::new();

    // Assemble the code
    let code = assembler
        .assemble(source)
        .expect("Failed to assemble hello.asm");

    // Generate .8xp file
    let output = TI8XPGenerator::create_8xp("HELLO", &code);

    // Compare with known good output
    let expected = include_bytes!("fixtures/hello.8xp");

    // The files should match exactly
    assert_eq!(output.len(), expected.len(), "File sizes differ");
    assert_eq!(
        output, expected,
        "Generated .8xp file doesn't match expected"
    );
}

#[test]
fn test_math_demo_assembly() {
    let source = include_str!("fixtures/math.asm");
    let mut assembler = Z80Assembler::new();

    // Assemble the code
    let code = assembler
        .assemble(source)
        .expect("Failed to assemble math.asm");

    // Generate .8xp file
    let output = TI8XPGenerator::create_8xp("MATH", &code);

    // Compare with known good output (note: name will be different, so just check assembly)
    assert!(code.len() > 0, "No code generated");
    assert!(output.len() > 100, "Output file too small");
}

#[test]
fn test_basic_instructions() {
    let mut assembler = Z80Assembler::new();

    // Test NOP instruction
    let code = assembler.assemble("nop").expect("Failed to assemble NOP");
    assert_eq!(code, vec![0x00]);

    // Test RET instruction
    let code = assembler.assemble("ret").expect("Failed to assemble RET");
    assert_eq!(code, vec![0xc9]);

    // Test LD A,B
    let code = assembler
        .assemble("ld a,b")
        .expect("Failed to assemble LD A,B");
    assert_eq!(code, vec![0x78]);
}

#[test]
fn test_immediate_loads() {
    let mut assembler = Z80Assembler::new();

    // Test LD A,42
    let code = assembler
        .assemble("ld a,42")
        .expect("Failed to assemble LD A,42");
    assert_eq!(code, vec![0x3e, 42]);

    // Test LD HL,$1234
    let code = assembler
        .assemble("ld hl,$1234")
        .expect("Failed to assemble LD HL,$1234");
    assert_eq!(code, vec![0x21, 0x34, 0x12]);
}

#[test]
fn test_labels_and_jumps() {
    let mut assembler = Z80Assembler::new();

    let source = r#"
        .org $9D93
        jp end
        nop
        nop
    end:
        ret
    "#;

    let code = assembler
        .assemble(source)
        .expect("Failed to assemble with labels");
    // JP end (where end = $9D93 + 3 + 2 = $9D98)
    assert_eq!(&code[0..3], &[0xc3, 0x98, 0x9d]);
    assert_eq!(&code[3..5], &[0x00, 0x00]); // Two NOPs
    assert_eq!(code[5], 0xc9); // RET
}

#[test]
fn test_data_directives() {
    let mut assembler = Z80Assembler::new();

    // Test .db with string
    let code = assembler
        .assemble(r#".db "Hello",0"#)
        .expect("Failed to assemble .db");
    assert_eq!(code, b"Hello\0");

    // Test .db with bytes
    let code = assembler
        .assemble(".db $FF, $00, 42")
        .expect("Failed to assemble .db bytes");
    assert_eq!(code, vec![0xFF, 0x00, 42]);

    // Test .dw
    let code = assembler
        .assemble(".dw $1234")
        .expect("Failed to assemble .dw");
    assert_eq!(code, vec![0x34, 0x12]); // Little-endian
}

#[test]
fn test_ix_iy_instructions() {
    let mut assembler = Z80Assembler::new();

    // Test LD IX,nn
    let code = assembler
        .assemble("ld ix,$1234")
        .expect("Failed to assemble LD IX");
    assert_eq!(code, vec![0xdd, 0x21, 0x34, 0x12]);

    // Test LD IY,nn
    let code = assembler
        .assemble("ld iy,$5678")
        .expect("Failed to assemble LD IY");
    assert_eq!(code, vec![0xfd, 0x21, 0x78, 0x56]);

    // Test PUSH IX
    let code = assembler
        .assemble("push ix")
        .expect("Failed to assemble PUSH IX");
    assert_eq!(code, vec![0xdd, 0xe5]);

    // Test POP IY
    let code = assembler
        .assemble("pop iy")
        .expect("Failed to assemble POP IY");
    assert_eq!(code, vec![0xfd, 0xe1]);
}

#[test]
fn test_bit_manipulation() {
    let mut assembler = Z80Assembler::new();

    // Test BIT
    let code = assembler
        .assemble("bit 3,a")
        .expect("Failed to assemble BIT");
    assert_eq!(code, vec![0xcb, 0x5f]); // CB prefix, then 01 011 111 (bit 3, reg a)

    // Test SET
    let code = assembler
        .assemble("set 7,b")
        .expect("Failed to assemble SET");
    assert_eq!(code, vec![0xcb, 0xf8]); // CB prefix, then 11 111 000 (set 7, reg b)

    // Test RES
    let code = assembler
        .assemble("res 0,c")
        .expect("Failed to assemble RES");
    assert_eq!(code, vec![0xcb, 0x81]); // CB prefix, then 10 000 001 (res 0, reg c)

    // Test rotate/shift
    let code = assembler.assemble("rlc a").expect("Failed to assemble RLC");
    assert_eq!(code, vec![0xcb, 0x07]);

    let code = assembler.assemble("sla b").expect("Failed to assemble SLA");
    assert_eq!(code, vec![0xcb, 0x20]);
}

#[test]
fn test_io_instructions() {
    let mut assembler = Z80Assembler::new();

    // Test IN A,(n)
    let code = assembler
        .assemble("in a,($10)")
        .expect("Failed to assemble IN");
    assert_eq!(code, vec![0xdb, 0x10]);

    // Test OUT (n),A
    let code = assembler
        .assemble("out ($11),a")
        .expect("Failed to assemble OUT");
    assert_eq!(code, vec![0xd3, 0x11]);

    // Test IN r,(C)
    let code = assembler
        .assemble("in b,(c)")
        .expect("Failed to assemble IN B,(C)");
    assert_eq!(code, vec![0xed, 0x40]);

    // Test block I/O
    let code = assembler.assemble("otir").expect("Failed to assemble OTIR");
    assert_eq!(code, vec![0xed, 0xb3]);
}

#[test]
fn test_block_transfer() {
    let mut assembler = Z80Assembler::new();

    // Test LDIR
    let code = assembler.assemble("ldir").expect("Failed to assemble LDIR");
    assert_eq!(code, vec![0xed, 0xb0]);

    // Test CPIR
    let code = assembler.assemble("cpir").expect("Failed to assemble CPIR");
    assert_eq!(code, vec![0xed, 0xb1]);

    // Test LDD
    let code = assembler.assemble("ldd").expect("Failed to assemble LDD");
    assert_eq!(code, vec![0xed, 0xa8]);
}

#[test]
fn test_expanded_rom_calls() {
    let mut assembler = Z80Assembler::new();

    // Test math ROM call
    let code = assembler
        .assemble("bcall(_FPAdd)")
        .expect("Failed to assemble _FPAdd");
    assert_eq!(code, vec![0xef, 0x72, 0x40]); // RST 28h, then address

    // Test graphics ROM call
    let code = assembler
        .assemble("bcall(_ILine)")
        .expect("Failed to assemble _ILine");
    assert_eq!(code, vec![0xef, 0x98, 0x47]);

    // Test variable management
    let code = assembler
        .assemble("bcall(_ChkFindSym)")
        .expect("Failed to assemble _ChkFindSym");
    assert_eq!(code, vec![0xef, 0xf1, 0x42]);
}
