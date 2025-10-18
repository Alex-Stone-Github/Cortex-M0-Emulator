use crate::core::*;
use crate::ins::{fetch_instruction, Instruction, InstructionClass};

#[derive(Debug, Clone)]
pub struct Registers {
    pub R: [AWord; 13], // General Purpose Registers
    pub SP: AWord,
    pub LR: AWord,
    pub PC: AWord,

    // Flags
    pub N: bool,
    pub Z: bool,
    pub C: bool,
    pub V: bool,

    // Special(could be memory accessed)
    // CPUID, ICSR, AIRCR, CCR, PRIMASK, CONTROL, CPSR
}

fn exec_thumb1(cpu: &mut Registers, ins: AHalfWord) {
    match ins {
        // ADC
        i if bitidx(i, 6, 10) == 0b0100000101 => {
            let rm = &mut cpu.R[bitidx(i, 3, 3) as usize];
            let rdn = &mut cpu.R[bitidx(i, 0, 3) as usize];
            println!("Executing adc reg {} {}", rm, rdn);
            *rdn += *rm + if cpu.C {1} else {0};
            cpu.N = bitidx(*rdn, 31, 1) == 1;
         },
        _ => {
            println!("Unimplemented Instruction");
        },
    }
}


pub fn execute(cpu: &mut Registers, memory: &[AByte]) {
    for _ in 0..(memory.len() / 2) {
        let instruction = fetch_instruction(&mut cpu.PC, memory);
        match instruction {
            Instruction::Thumb1(ins) => exec_thumb1(cpu, ins),
            Instruction::Thumb2(_, _) => unimplemented!()
        }
    }
}
