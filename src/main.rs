mod fetch;
mod core;
mod ins;
mod registers;
mod instructions;
mod adr;
mod fstools;
mod memory;
mod config;

use crate::{adr::AddressSpace, fetch::fetch_instruction, instructions::load_basic_instructions, registers::{PC_IDX, SP_IDX}};

fn print_proc_state(cpu: &registers::Registers) {
    log::debug!("New CPU State: PC({}), R0-R7({}, {}, {}, {}, {}, {}, {}, {})",
        cpu.r[PC_IDX], cpu.r[0], cpu.r[1], cpu.r[2], cpu.r[3], cpu.r[4], cpu.r[5], cpu.r[6], cpu.r[7]
        )
}

fn main() {
    env_logger::init();

    log::info!("Loading Config");
    let mut address_space = config::load();
    log::info!("Loaded Config");

    let mut cpu = &mut registers::Registers {
        r: [0; 16],
        n: false,
        z: false,
        c: false,
        v: false,
    };
    cpu.r[PC_IDX] = 2; // PC Points to currently executing instruction + 4
    cpu.r[SP_IDX] = 16; // kinda a hack to start with


    // Implement Instructions
    let mut instructions = ins::LoaderExecuter::new();
    load_basic_instructions(&mut instructions);

    // Run the program
    let instruction_count = 2000;
    for _ in 0..instruction_count {
        step(&instructions, &mut cpu, &mut *address_space);
        print_proc_state(&cpu);
    }
}

pub fn step(
    supported_instructions: &ins::LoaderExecuter,
    cpu: &mut registers::Registers,
    addresses: &mut dyn AddressSpace) {
    let instruction = fetch_instruction(&mut cpu.r[registers::PC_IDX], addresses);
    supported_instructions.execute(&instruction, cpu, addresses);
}
