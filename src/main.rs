mod fetch;
mod core;
mod ins;
mod registers;

use crate::{core::*, fetch::fetch_instruction};

fn main() {
    let mut memory = [
        // Shift Type Stuff
        0, 0,          // Shift LSL
        0b00001000, 0, // Shift LSR
        0b00010000, 0, // Shift ASR
        0b00011000, 0, // Add Reg
        0b00011010, 0, // Sub Reg
        0b00011100, 0, // Add 3bit Immediate
        0b00011110, 0, // Sub 3bit Immediate
        0b00100000, 0, // Move
        0b00101000, 0, // Compare
        0b00110000, 0, // Add 8bit imed
        0b00111000, 0, // sub 8bit imed
                       
                       
        0b01000001, 0b01111000, // Data Processing
        0b01000100, 0, // Special Data
        0b01001000, 0, // Load Literal
        0b01010000, 0, // Ldr / Str
        0b01100000, 0, // Ldr / Str
        0b10000000, 0, // Ldr / Str
        0b10100000, 0, // Generate PC Adr
        0b10101000, 0, // Generate SP Adr
        0b10110000, 0, // Misc
        0b11000000, 0, // Store Regs
        0b11001000, 0, // Load Regs
        0b11010000, 0, // Cond Branch
        0b11100000, 0, // UnCond Branch
    ];
    let mut cpu = &mut registers::Registers {
        r: [0; 13],
        sp: 0,
        lr: 0,
        pc: 0,

        n: false,
        z: false,
        c: false,
        v: false,
    };

    // Implement Instructions
    let mut instructions = ins::LoaderExecuter::new();

    instructions.implement(
        "ADC (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 9) == 0b0100000101,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] += cpu.r[rm_no] + if cpu.c {1} else {0};
        }
    );

    instructions.implement(
        "Add (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001110,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 3) as AWord;
            cpu.r[rd_no] = cpu.r[rn_no] + imd;
        }
    );

    instructions.implement(
        "Add (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001100,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            cpu.r[rd_no] = cpu.r[rm_no] + cpu.r[rn_no];
        }
    );

    instructions.implement(
        "Add (SP + immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10101,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            cpu.r[rd_no] = cpu.sp + imd;
        }
    );

    instructions.implement(
        "Add (SP + register)",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000100 && ins.hdr.idx(3, 4) == 0b1101,
        |ins, cpu, _| {
            let rdm_no = ins.hdr.idx(0, 3) as usize;
            cpu.r[rdm_no] += cpu.sp;
        }
    );

    instructions.implement(
        "ADR",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10100,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            cpu.r[rd_no] = cpu.pc + imd;
        }
    );

    instructions.implement(
        "AND",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000000,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] &= cpu.r[rm_no];
        }
    );

    instructions.implement(
        "ASR (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00010,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            cpu.r[rd_no] = cpu.r[rm_no] >> imd;
        }
    );

    loop {
        let instruction = fetch_instruction(&mut cpu.pc, &memory);
        instructions.execute(&instruction, &mut cpu, &mut memory);
        dbg!(&cpu);
    }
}
