mod fetch;
mod core;
mod ins;
mod registers;
mod instructions;
mod adr;

use crate::{core::*, fetch::fetch_instruction, registers::{PC_IDX, SP_IDX}};
use crate::instructions::load_basic_instructions;

fn main() {
    let mut cpu = &mut registers::Registers {
        r: [0; 16],
        n: false,
        z: false,
        c: false,
        v: false,
    };
    let mut memory = adr::Sample(&adr::SAMPLE);

    // Implement Instructions
    let mut instructions = ins::LoaderExecuter::new();
    load_basic_instructions(
        &mut instructions,
        &mut cpu,
        &mut memory
    );

    // Run the program
    loop {
        let instruction = fetch_instruction(&mut cpu.r[registers::PC_IDX], &mut memory);
        instructions.execute(&instruction, &mut cpu, &mut memory);
    }
}
