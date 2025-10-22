mod fetch;
mod core;
mod ins;
mod registers;
mod instructions;
mod adr;
mod fstools;
mod memory;
mod config;

use std::ops::DerefMut;

use crate::{core::*, fetch::fetch_instruction, registers::{PC_IDX, SP_IDX}};
use crate::instructions::load_basic_instructions;

fn print_proc_state(cpu: &registers::Registers) {
    println!("New CPU State: PC({}), R0-R7({}, {}, {}, {}, {}, {}, {}, {})",
        cpu.r[PC_IDX], cpu.r[0], cpu.r[1], cpu.r[2], cpu.r[3], cpu.r[4], cpu.r[5], cpu.r[6], cpu.r[7]
        )
}

fn main() {
    config::load();


    let mut cpu = &mut registers::Registers {
        r: [0; 16],
        n: false,
        z: false,
        c: false,
        v: false,
    };
    cpu.r[PC_IDX] = 2; // PC Points to currently executing instruction + 4
    cpu.r[SP_IDX] = 16; // kinda a hack to start with

    const PATH: &str = "./build/program";
    let mut memory = fstools::read_file_buffer(PATH).expect(&format!("Could not load {}", PATH));
    let mut address_space = memory::BufferMemory{
        origin: 0,
        buffer: memory.deref_mut(),
    };

    // Implement Instructions
    let mut instructions = ins::LoaderExecuter::new();
    load_basic_instructions(&mut instructions);

    // Run the program
    println!("Press Enter to step the program!");
    let stdin = std::io::stdin();
    let mut tmp = String::new();
    loop {
        println!("READY!-------------------------------------");
        let instruction = fetch_instruction(&mut cpu.r[registers::PC_IDX], &mut address_space);
        dbg!(&address_space.buffer);
        instructions.execute(&instruction, &mut cpu, &mut address_space);
        print_proc_state(&cpu);
        stdin.read_line(&mut tmp).expect("Stdout Error"); // Wait for next step
    }
}
