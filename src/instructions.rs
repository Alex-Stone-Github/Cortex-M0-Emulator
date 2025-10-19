use crate::adr::AddressSpace;
use crate::registers::{Registers, SP_IDX, LR_IDX, PC_IDX};
use crate::core::*;
use crate::ins::LoaderExecuter;


const DATA_IS_LE: bool = true;

// signit and unsignit exist so we don't mess with the sign bit on implicit conversions
fn signit(x: AWord) -> i32 {
    // Defined on x86_64
    unsafe { std::mem::transmute::<u32, i32>(x) }
}
// signit and unsignit exist so we don't mess with the sign bit on implicit conversions
fn unsignit(x: i32) -> AWord {
    // Defined on x86_64
    unsafe { std::mem::transmute::<i32, u32>(x) }
}

/// Result, Carry Out, Overflow
fn cortex_add(a: AWord, b: AWord) -> (AWord, bool, bool) {
    let (result, carry_out) = a.overflowing_add(b);
    let (_, overflow) = signit(a).overflowing_add(signit(b));
    (result, carry_out, overflow)
}
fn cortex_sub(a: AWord, b: AWord) -> (AWord, bool, bool) {
    let (result, carry_out) = a.overflowing_sub(b);
    let (_, overflow) = signit(a).overflowing_sub(signit(b));
    (result, carry_out, overflow)
}

pub fn load_basic_instructions(
    instructions: &mut LoaderExecuter,
    cpu: &mut Registers,
    addresses: &mut dyn AddressSpace) {

    // Start Instruction Definitions
    instructions.implement(
        "ADC (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 9) == 0b0100000101,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let (result, carry1, over1) = cortex_add(cpu.r[rm_no], if cpu.c {1} else {0});
            let (result2, carry2, over2) = cortex_add(result, cpu.r[rdn_no]);
            cpu.r[rdn_no] = result2;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.z = cpu.r[rdn_no] == 0;
            cpu.c = carry1 || carry2;
            cpu.v = over1 || over2;
        }
    );

    instructions.implement(
        "Add (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001110,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 3) as AWord;
            let (result, carry, over) = cortex_add(cpu.r[rm_no], imd);
            cpu.r[rd_no] = result;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "Add (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001100,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let rn_no = ins.hdr.idx(6, 3) as usize;
            let (result, carry, over) = cortex_add(cpu.r[rm_no], cpu.r[rn_no]);
            cpu.r[rd_no] = result;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "Add (SP + immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10101,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            let (result, carry, over) = cortex_add(cpu.r[SP_IDX], imd);
            cpu.r[rd_no] = result;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "Add (SP + register)",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000100 && ins.hdr.idx(3, 4) == 0b1101,
        |ins, cpu, _| {
            let rdm_no = ins.hdr.idx(0, 3) as usize;
            let (result, carry, over) = cortex_add(cpu.r[SP_IDX], cpu.r[rdm_no]);
            cpu.r[rdm_no] = result;
            cpu.n = 0 < (cpu.r[rdm_no] & (1 << 31));
            cpu.z = cpu.r[rdm_no] == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "ADR",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10100,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            let (result, carry, over) = cortex_add(cpu.r[PC_IDX], imd);
            cpu.r[rd_no] = result;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "AND",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000000,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] &= cpu.r[rm_no];
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.z = cpu.r[rdn_no] == 0;
        }
    );

    instructions.implement(
        "ASR (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00010,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            cpu.r[rd_no] = cpu.r[rm_no] >> (imd-1);
            let carry = 0 < (cpu.r[rd_no] & 1);
            cpu.r[rd_no] = cpu.r[rm_no] >> 1;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
        }
    );

    instructions.implement(
        "ASR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000100,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] = cpu.r[rdn_no] >> (cpu.r[rm_no]-1);
            let carry = 0 < (cpu.r[rdn_no] & 1);
            cpu.r[rdn_no] = cpu.r[rdn_no] >> 1;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.z = cpu.r[rdn_no] == 0;
            cpu.c = carry;
        }
    );

    instructions.implement(
        "B",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b1101,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let cond = ins.hdr.idx(8, 4) as usize;
            let should_branch = match cond {
                0b1110 => true,
                0b0000 => cpu.z == true,
                0b0001 => cpu.z == false,
                0b0010 => cpu.c == true,
                0b0011 => cpu.c == false,
                0b0100 => cpu.n == true,
                0b0101 => cpu.n == false,
                0b0110 => cpu.v == true,
                0b0111 => cpu.v == false,
                0b1000 => cpu.c == true && cpu.z == false,
                0b1001 => cpu.v == false && cpu.z == true,
                0b1010 => cpu.n == cpu.v,
                0b1011 => cpu.n != cpu.v,
                0b1100 => cpu.z == false && cpu.n == cpu.v,
                0b1101 => cpu.z == true && cpu.z != cpu.v,
                _ => false
            };
            if should_branch {
                cpu.r[PC_IDX] += imd;
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
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.z = cpu.r[rdn_no] == 0;
        }
    );

    instructions.implement(
        "BKPT",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b10111110,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            unimplemented!("Breakpoints are not implemented yet!")
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
            let imd10 = ins.hdr.idx(0, 10) as AWord;
            let s = ins.hdr.idx(10, 1) as AWord;
            let imd11 = ext.idx(0, 13) as AWord;
            let j1 = ext.idx(13, 1) as AWord;
            let j2 = ext.idx(11, 1) as AWord;
            let i1 = if !((j1 > 0) ^ (s > 0)) {1} else {0};
            let i2 = if !((j2 > 0) ^ (s > 0)) {1} else {0};
            let mut address: AWord = 0;
            address |= imd11;
            address |= (imd10 << 11);
            address |= (i1 << 21);
            address |= (i2 << 22);
            address |= (s << 23);
            address = address << 1; // 25th "bit"
            // Save PC to LR
            cpu.r[LR_IDX] = cpu.r[PC_IDX];
            cpu.r[PC_IDX] = address;
            
        }
    );

    instructions.implement(
        "BLX (register)",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b010001111,
        |ins, cpu, _| {
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[LR_IDX] = cpu.r[PC_IDX];
            cpu.r[PC_IDX] = cpu.r[rm_no];
        }
    );

    // Very similar(basically) -> Branch (register)
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
            let (result, carry, over) = cortex_add(cpu.r[rm_no], cpu.r[rn_no]);
            cpu.n = 0 < (result & (1 << 31));
            cpu.z = result == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "CMP (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00101,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rn_no = ins.hdr.idx(8, 3) as usize;
            let (result, carry, over) = cortex_sub(cpu.r[rn_no], imd);
            cpu.n = 0 < (result & (1 << 31));
            cpu.z = result == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "CMP (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001010,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let (result, carry, over) = cortex_sub(cpu.r[rn_no], cpu.r[rm_no]);
            cpu.n = 0 < (result & (1 << 31));
            cpu.z = result == 0;
            cpu.c = carry;
            cpu.v = over;
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
            unimplemented!("Data Memory Barrier Unimplemented")
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
            unimplemented!("Data Sync Barrier Unimplemented")
        }
    );

    instructions.implement(
        "EOR",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000001,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] ^= cpu.r[rm_no];
            let result = cpu.r[rdn_no];
            cpu.n = 0 < (result & (1 << 31));
            cpu.z = result == 0;
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
            unreachable!("Instruction sync barrier unimplmeneted")
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

}
