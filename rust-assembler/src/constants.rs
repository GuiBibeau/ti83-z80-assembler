//! Constants used throughout the Z80 assembler
//!
//! This module contains all magic numbers and constants to ensure
//! consistency and maintainability across the codebase.

// TI-83 Plus specific constants
/// Default origin address for TI-83 Plus programs
pub const TI83_PLUS_ORIGIN: u16 = 0x9D93;

/// AsmPrgm header bytes for TI-83 Plus
pub const ASM_PRGM_HEADER: [u8; 2] = [0xBB, 0x6D];

/// Maximum program name length for TI-83 Plus
pub const MAX_PROGRAM_NAME_LENGTH: usize = 8;

// Z80 CPU specific constants
/// RST 28h instruction for bcall
pub const RST_28H: u8 = 0xEF;

/// Maximum displacement for indexed addressing (IX+d, IY+d)
pub const MAX_INDEX_DISPLACEMENT: i8 = 127;
pub const MIN_INDEX_DISPLACEMENT: i8 = -128;

/// Maximum relative jump distance
pub const MAX_RELATIVE_JUMP: i32 = 127;
pub const MIN_RELATIVE_JUMP: i32 = -128;

// Instruction prefixes
/// IX register prefix
pub const IX_PREFIX: u8 = 0xDD;

/// IY register prefix
pub const IY_PREFIX: u8 = 0xFD;

/// Bit manipulation prefix
pub const CB_PREFIX: u8 = 0xCB;

/// Extended instruction prefix
pub const ED_PREFIX: u8 = 0xED;

// File format constants
/// TI-83 Plus file signature
pub const TI83_FILE_SIGNATURE: &[u8] = b"**TI83F*";

/// TI-83 Plus file header size
pub const FILE_HEADER_SIZE: usize = 55;

// Display constants
/// TI-83 Plus screen width in pixels
pub const SCREEN_WIDTH: u8 = 96;

/// TI-83 Plus screen height in pixels
pub const SCREEN_HEIGHT: u8 = 64;

// Memory constants
/// Safe RAM area start address
pub const SAFE_RAM_START: u16 = 0x8000;

/// Program data area start
pub const PROGRAM_DATA_START: u16 = 0x9D95;

// I/O Port constants
/// Link port address
pub const LINK_PORT: u8 = 0x00;

/// Keyboard port address
pub const KEYBOARD_PORT: u8 = 0x01;

/// LCD command port
pub const LCD_COMMAND_PORT: u8 = 0x10;

/// LCD data port
pub const LCD_DATA_PORT: u8 = 0x11;
