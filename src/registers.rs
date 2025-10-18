use crate::core::*;

pub const SP_IDX: usize = 13;
pub const LR_IDX: usize = 14;
pub const PC_IDX: usize = 15;

#[derive(Debug, Clone)]
pub struct Registers {
    pub r: [AWord; 16], // General Purpose Registers

    // Flags
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,

    // Special(could be memory accessed)
    // CPUID, ICSR, AIRCR, CCR, PRIMASK, CONTROL, CPSR
}
