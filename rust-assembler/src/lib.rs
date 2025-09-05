//! Z80 Assembler for TI-83 Plus Calculator Programs
//!
//! This crate provides a complete Z80 assembler implementation
//! specifically designed for creating TI-83 Plus calculator programs.
//!
//! # Features
//! - Full Z80 instruction set support
//! - TI-83 Plus specific ROM calls
//! - Label and constant support
//! - Generates valid .8xp files

pub mod assembler;
pub mod constants;
pub mod directives;
pub mod instructions;
pub mod ti83plus;
pub mod utils;

pub use assembler::Z80Assembler;
pub use ti83plus::TI8XPGenerator;
