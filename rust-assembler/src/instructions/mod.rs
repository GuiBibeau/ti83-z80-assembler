pub mod arithmetic;
pub mod bitops;
pub mod calls;
pub mod index;
pub mod io;
pub mod jumps;
pub mod loads;
pub mod opcodes;

pub use arithmetic::handle_arithmetic_instruction;
pub use bitops::handle_bit_instruction;
pub use calls::handle_call_instruction;
pub use index::handle_index_instruction;
pub use io::handle_io_instruction;
pub use jumps::handle_jump_instruction;
pub use loads::handle_load_instruction;
