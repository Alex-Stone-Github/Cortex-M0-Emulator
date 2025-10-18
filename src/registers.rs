use crate::core::*;
use crate::ins::{Instruction, fetch_instruction};

#[derive(Debug, Clone)]
pub struct Registers {
    pub R: [AWord; 13], // General Purpose Registers
    pub SP: AWord,
    pub LR: AWord,
    pub PC: AWord,

    // Special(could be memory accessed)
    // CPUID, ICSR, AIRCR, CCR, PRIMASK, CONTROL, CPSR
}

fn exec_shift_class(cpu: &mut Registers, ins: Instruction) {

}

pub fn execute(cpu: &mut Registers, memory: &[AByte]) {
    for _ in 0..13 {
        let instruction = fetch_instruction(&mut cpu.PC, memory);
        dbg!(instruction.get_class());
    }
}
