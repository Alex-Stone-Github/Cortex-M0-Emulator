mod fetch;
mod core;
mod ins;
mod registers;

use crate::{core::*, fetch::fetch_instruction, registers::{PC_IDX, SP_IDX}};

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
        r: [0; 16],
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
            cpu.r[rd_no] = cpu.r[SP_IDX] + imd;
        }
    );

    instructions.implement(
        "Add (SP + register)",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000100 && ins.hdr.idx(3, 4) == 0b1101,
        |ins, cpu, _| {
            let rdm_no = ins.hdr.idx(0, 3) as usize;
            cpu.r[rdm_no] += cpu.r[SP_IDX];
        }
    );

    instructions.implement(
        "ADR",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10100,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            cpu.r[rd_no] = cpu.r[PC_IDX] + imd;
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

    instructions.implement(
        "ASR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000100,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] = cpu.r[rdn_no] >> cpu.r[rm_no];
        }
    );

    instructions.implement(
        "B",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b1101,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let cond_no = ins.hdr.idx(8, 4) as usize;
            if cond_no == 0 {
                cpu.r[PC_IDX] += imd;
            }
            else {
                unimplemented!("don't support conditional branching yet")
            }
        }
    );

    instructions.implement(
        "BIC (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001110,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] &= cpu.r[rm_no].wrapping_neg();
        }
    );

    instructions.implement(
        "BKPT",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b10111110,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            println!("BREAKPOINT HITQQQQQQQQQ!!!!!!!!!!!!!");
        }
    );

    instructions.implement(
        "BL",
        |ins| {
            let thumb1 = ins.is_t1();
            if thumb1 {return false;}
            let first_part_good = ins.hdr.idx(11, 5) == 0b11110;
            let second_part_good = ins.ext.unwrap().idx(14, 2) == 0b11;
            return !thumb1 && first_part_good && second_part_good;
        },
        |ins, cpu, _| {
            let ext = ins.ext.unwrap();
            cpu.r[PC_IDX] = 0;
            todo!()
        }
    );

    instructions.implement(
        "BLX (register)",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b010001111,
        |ins, cpu, _| {
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[PC_IDX] = cpu.r[rm_no];
        }
    );

    instructions.implement(
        "BX",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b010001110,
        |ins, cpu, _| {
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[PC_IDX] = cpu.r[rm_no];
        }
    );

    instructions.implement(
        "CMN (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001011,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            unimplemented!("Should do the same as an ads but just updates flags");
        }
    );

    instructions.implement(
        "CMP (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00101,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as usize;
            let rn_no = ins.hdr.idx(8, 3) as usize;
            unimplemented!("should do same as the sub but discoards rust and just upadate flags");
        }
    );

    instructions.implement(
        "CMP (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001010,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            unimplemented!("should do same as the sub but discoards rust and just upadate flags");
        }
    );

    instructions.implement(
        "CMP (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001010,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            unimplemented!("should do same as the sub but discoards rust and just upadate flags");
        }
    );

    instructions.implement(
        "DMB",
        |ins| {
            let thumb1 = ins.is_t1();
            if thumb1 {return false;}
            let first_part_good = ins.hdr.idx(0, 16) == 0b1111001110111111;
            let second_part_good = ins.ext.unwrap().idx(12, 4) == 0b100011110101;
            return !thumb1 && first_part_good && second_part_good;
        },
        |ins, cpu, _| {
            let ext = ins.ext.unwrap();
            // Data Memory Barrier, acts as a memory barrier iensures that explicit memory access
            // appears before this function is called
            todo!()
        }
    );

    instructions.implement(
        "DSB",
        |ins| {
            let thumb1 = ins.is_t1();
            if thumb1 {return false;}
            let first_part_good = ins.hdr.idx(0, 16) == 0b1111001110111111;
            let second_part_good = ins.ext.unwrap().idx(12, 4) == 0b100011110100;
            return !thumb1 && first_part_good && second_part_good;
        },
        |ins, cpu, _| {
            let ext = ins.ext.unwrap();
            // Data Sync Barrier, acts as a memory barrier iensures that explicit memory access
            // appears before this function is called same thing as dmb but with instructions
            todo!()
        }
    );

    instructions.implement(
        "EOR",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000001,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] ^= cpu.r[rm_no];
        }
    );

    instructions.implement(
        "ISB",
        |ins| {
            let thumb1 = ins.is_t1();
            if thumb1 {return false;}
            let first_part_good = ins.hdr.idx(0, 16) == 0b1111001110111111;
            let second_part_good = ins.ext.unwrap().idx(12, 4) == 0b100011110110;
            return !thumb1 && first_part_good && second_part_good;
        },
        |ins, cpu, _| {
            let ext = ins.ext.unwrap();

            // Instruction Sync Barrier, acts as a memory barrier iensures that explicit memory
            // access so thae appears before this function is called same thing as dmb but with
            // instructions this is actually a prety cool system of text, it will be cool when I
            // eventually implement what these should actually do
            
            todo!()
        }
    );

    instructions.implement(
        "LDM, LMIA, LDMFD",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b11001,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(8, 3) as usize;
            let reglist = ins.hdr.idx(0, 8) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDR (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01101,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDR (literal)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01001,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rt_no = ins.hdr.idx(8, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101100,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDRB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01111,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDRB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101110,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDRH (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10001,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDRSB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101011,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "LDRSH (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101111,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "LSL (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00000,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            cpu.r[rd_no] = cpu.r[rm_no] << imd;
        }
    );

    instructions.implement(
        "LSL (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000010,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] = cpu.r[rdn_no] << cpu.r[rm_no];
        }
    );

    instructions.implement(
        "LSR (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00001,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            cpu.r[rd_no] = cpu.r[rm_no] >> imd;
        }
    );

    instructions.implement(
        "LSR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000011,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] = cpu.r[rdn_no] >> cpu.r[rm_no];
        }
    );

    instructions.implement(
        "MOV (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00100,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            cpu.r[rd_no] = imd;
        }
    );

    instructions.implement(
        "MOV (register)",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000110,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 4) as usize;
            cpu.r[rd_no] = cpu.r[rm_no];
        }
    );

    instructions.implement(
        "MRS",
        |ins| {
            let thumb1 = ins.is_t1();
            if thumb1 {return false;}
            let first_part_good = ins.hdr.idx(0, 16) == 0b1111001111101111;
            let second_part_good = ins.ext.unwrap().idx(12, 4) == 1000;
            return !thumb1 && first_part_good && second_part_good;
        },
        |ins, cpu, _| {
            let ext = ins.ext.unwrap();

            // Instruction Sync Barrier, acts as a memory barrier iensures that explicit memory
            // access so thae appears before this function is called same thing as dmb but with
            // instructions this is actually a prety cool system of text, it will be cool when I
            // eventually implement what these should actually do
            
            todo!()
        }
    );

    instructions.implement(
        "MSR (register)",
        |ins| {
            let thumb1 = ins.is_t1();
            if thumb1 {return false;}
            let first_part_good = ins.hdr.idx(4, 12) == 0b111100111000;
            let second_part_good = ins.ext.unwrap().idx(8, 8) == 0b10001000;
            return !thumb1 && first_part_good && second_part_good;
        },
        |ins, cpu, _| {
            let ext = ins.ext.unwrap();

            // Instruction Sync Barrier, acts as a memory barrier iensures that explicit memory
            // access so thae appears before this function is called same thing as dmb but with
            // instructions this is actually a prety cool system of text, it will be cool when I
            // eventually implement what these should actually do
            
            todo!()
        }
    );

    instructions.implement(
        "MUL",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001101,
        |ins, cpu, _| {
            let rdm_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdm_no] *= cpu.r[rn_no];
        }
    );

    instructions.implement(
        "MVN",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001101,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = !cpu.r[rn_no];
        }
    );

    instructions.implement(
        "NOP",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100000000,
        |_, _, _| {}
    );

    instructions.implement(
        "ORR",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001100,
        |ins, cpu, _| {
            let rdm_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdm_no] |= cpu.r[rn_no];
        }
    );

    instructions.implement(
        "POP",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b1011110,
        |ins, cpu, _| {
            todo!()
        }
    );

    instructions.implement(
        "PUSH",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b10110110,
        |ins, cpu, _| {
            todo!()
        }
    );

    instructions.implement(
        "REV",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b10111010,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = cpu.r[rn_no].swap_bytes();
        }
    );

    instructions.implement(
        "REV16",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b10111010,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = (cpu.r[rn_no] as AHalfWord).swap_bytes() as AWord;
        }
    );

    instructions.implement(
        "REVSH",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011101011,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            todo!("need to make sure sign is presenved");
            cpu.r[rd_no] = (cpu.r[rn_no] as AHalfWord).swap_bytes() as AWord;
        }
    );

    instructions.implement(
        "ROR",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000111,
        |ins, cpu, _| {
            let rdm_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdm_no] = cpu.r[rdm_no].rotate_right(cpu.r[rn_no]);
        }
    );

    instructions.implement(
        "RSB (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001001,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            todo!();
        }
    );

    instructions.implement(
        "SBC (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000110,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            todo!();
        }
    );

    instructions.implement(
        "SEV",
        |ins| ins.is_t1() && ins.hdr == 0b1011111101000000,
        |_, _, _| {
            // send event hint instruction
            todo!();
        }
    );

    instructions.implement(
        "STM, STMIA, STMEA",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b11000,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(8, 3) as usize;
            let reglist = ins.hdr.idx(0, 8) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "STR (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01100,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "STR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101000,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "STRB (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01110,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "STRB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101010,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "STRH (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10001,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "STRH (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101001,
        |ins, cpu, _| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            unimplemented!()
        }
    );

    instructions.implement(
        "SUB (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001111,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 3) as AWord;
            cpu.r[rd_no] = cpu.r[rm_no] - imd;
        }
    );

    instructions.implement(
        "SUB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001101,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let rn_no = ins.hdr.idx(6, 3) as usize;
            cpu.r[rd_no] = cpu.r[rm_no] - cpu.r[rn_no];
        }
    );

    instructions.implement(
        "SUB (SP minus intermediate)",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b10110001,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 7) as AWord;
            cpu.r[SP_IDX] -= imd;
        }
    );

    instructions.implement(
        "SVC",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b11011111,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 7) as AWord;
            todo!()
        }
    );

    instructions.implement(
        "SXTB",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001001,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            todo!()
        }
    );

    instructions.implement(
        "SXTH",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001000,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            todo!()
        }
    );

    instructions.implement(
        "TST (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001000,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            // performs add, sets flags, discards results
            todo!()
        }
    );

    instructions.implement(
        "UDF",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b11011110,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as usize;
            // undefined instruction exception generation
            todo!()
        }
    );

    instructions.implement(
        "UXTB",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001011,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            todo!()
        }
    );

    instructions.implement(
        "UXTH",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001010,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            todo!()
        }
    );

    instructions.implement(
        "WFE",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100100000,
        |ins, cpu, _| {
            // Enter Low Power Mode until event is  sent
            todo!()
        }
    );

    instructions.implement(
        "WFI",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100110000,
        |ins, cpu, _| {
            // Wait for interrupt
            todo!()
        }
    );

    instructions.implement(
        "YIELD",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100010000,
        |ins, cpu, _| {
            // yield enables multithreading capability
            todo!()
        }
    );

    loop {
        let instruction = fetch_instruction(&mut cpu.r[registers::PC_IDX], &memory);
        instructions.execute(&instruction, &mut cpu, &mut memory);
    }
}
