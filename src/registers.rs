use crate::core::*;

#[derive(Debug, Clone)]
pub struct Registers {
    pub r: [AWord; 13], // General Purpose Registers
    pub sp: AWord,
    pub lr: AWord,
    pub pc: AWord,

    // Flags
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,

    // Special(could be memory accessed)
    // CPUID, ICSR, AIRCR, CCR, PRIMASK, CONTROL, CPSR
}
