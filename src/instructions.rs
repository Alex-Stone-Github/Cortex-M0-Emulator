use crate::registers::{SP_IDX, LR_IDX, PC_IDX};
use crate::core::*;
use crate::ins::LoaderExecuter;

// todo: remove
/// signit and unsignit exist so we don't mess with the sign bit on implicit conversions
fn signit(x: AWord) -> i32 {
    // Defined on x86_64
    x as i32
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

pub fn load_basic_instructions(instructions: &mut LoaderExecuter) {

    // Little Endian & Big endian Loads and Stores

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
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001110;
            let t2 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b00110;
            t1 || t2
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001110;
            if t1 { // T1 Encoding
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
            if !t1 {
                let imd8 = ins.hdr.idx(0, 8) as AWord;
                let rdn_no = ins.hdr.idx(8, 3) as usize;
                let (result, carry, over) = cortex_add(cpu.r[rdn_no], imd8);
                cpu.r[rdn_no] = result;
                cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
                cpu.z = cpu.r[rdn_no] == 0;
                cpu.c = carry;
                cpu.v = over;
            }
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
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b10101;
            let t2 = ins.is_t1() && ins.hdr.idx(7, 9) == 0b101100000;
            t1 || t2
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b10101;
            if t1 { // T1 Encoding
                let imd = ins.hdr.idx(0, 8) as AWord;
                let rd_no = ins.hdr.idx(8, 3) as usize;
                let (result, _, _) = cortex_add(cpu.r[SP_IDX], imd);
                cpu.r[rd_no] = result;
            }
            if !t1 {
                let imd = ins.hdr.idx(0, 7) as AWord;
                let (result, _, _) = cortex_add(cpu.r[SP_IDX], imd << 2);
                cpu.r[SP_IDX] = result;
                
            }
        }
    );

    instructions.implement(
        "Add (SP + register)",
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000100 && ins.hdr.idx(3, 4) == 0b1101;
            let t2 = ins.is_t1() && ins.hdr.idx(7, 9) == 0b010001001 && ins.hdr.idx(0, 3) == 0b101;
            t1 || t2
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000100 && ins.hdr.idx(3, 4) == 0b1101;
            if t1 {
                // rdm, because technbially sp is arg 1
                let rdm_no = ins.hdr.idx(0, 3) as usize;
                let (result, _, _) = cortex_add(cpu.r[SP_IDX], cpu.r[rdm_no]);
                cpu.r[rdm_no] = result;
            }
            if !t1 {
                let rm_no = ins.hdr.idx(3, 4) as usize;
                let (result, _, _) = cortex_add(cpu.r[SP_IDX], cpu.r[rm_no]);
                cpu.r[SP_IDX] = result;
            }
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
            if 0 < imd {
                cpu.r[rd_no] = ((cpu.r[rm_no] as i32) >> (imd-1)) as AWord;
                cpu.c = 0 < (cpu.r[rd_no] & 1);
                cpu.r[rd_no] = ((cpu.r[rm_no]) >> 1) as AWord;
            } // shift 0 does nothing
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
        }
    );

    instructions.implement(
        "ASR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000100,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            if 0 < cpu.r[rm_no] {
                cpu.r[rdn_no] = ((cpu.r[rdn_no] as i32) >> (cpu.r[rm_no]-1)) as AWord;
                cpu.c = 0 < (cpu.r[rdn_no] & 1);
                cpu.r[rdn_no] = ((cpu.r[rdn_no] as i32) >> 1) as AWord;
            } // shift 0 doe snothing
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.z = cpu.r[rdn_no] == 0;
        }
    );

    //todo!("many instructions have multiple encodings: eg t2")

    instructions.implement(
        "B",
        |ins| {
            let t1 = ins.is_t1() && (ins.hdr.idx(12, 4) == 0b1101);
            let t2 = ins.is_t1() && (ins.hdr.idx(11, 5) == 0b11100);
            return t1 || t2;
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && (ins.hdr.idx(12, 4) == 0b1101);
            if t1 { // T1 Encoding
                let imd = ins.hdr.idx(0, 8) as AByte as i8 as i32 as AWord;
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
                    cpu.r[PC_IDX] = cpu.r[PC_IDX].wrapping_add(imd << 1).wrapping_add(2);
                }
            }
            else {
                // Don't touch the next lines unless kyou know what you are doing
                let imd11: i16 = ins.hdr.idx(0, 11) as i16;
                let imd11_sign_ext: i16 = ((imd11 << 5) >> 5) as i16;
                let imdoff: i32 = (imd11_sign_ext as i32) << 1;
                cpu.r[PC_IDX] = cpu.r[PC_IDX].wrapping_add(imdoff as AWord).wrapping_add(2);
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
        |ins, _cpu, _| {
            let _imd = ins.hdr.idx(0, 8) as AWord;
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
            let imd11 = ext.idx(0, 11) as AWord;
            let j1 = ext.idx(13, 1) as AWord;
            let j2 = ext.idx(11, 1) as AWord;
            let i1 = if !((j1 > 0) ^ (s > 0)) {1} else {0};
            let i2 = if !((j2 > 0) ^ (s > 0)) {1} else {0};
            let mut raw: AWord = 0;
            raw |= imd11;
            raw |= imd10 << 11;
            raw |= i1 << 21;
            raw |= i2 << 22;
            raw |= s << 23; // bit 24
            raw = raw << 1; // 25th "bit"
            let rawi: i32 = raw as i32;
            let address = (rawi << 7) >> 7;
            // Save PC to LR
            // ADR of next instruction
            cpu.r[LR_IDX] = cpu.r[PC_IDX];
            cpu.r[PC_IDX] = cpu.r[PC_IDX].wrapping_add(address as AWord);
            
        }
    );

    instructions.implement(
        "BLX (register)",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b010001111,
        |ins, cpu, _| {
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[LR_IDX] = cpu.r[PC_IDX] + 2;
            cpu.r[PC_IDX] = cpu.r[rm_no];
        }
    );

    // Very similar(basically) -> Branch (register)
    instructions.implement(
        "BX",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b010001110,
        |ins, cpu, _| {
            let rm_no = ins.hdr.idx(3, 4) as usize;
            let address = cpu.r[rm_no];
            cpu.r[PC_IDX] = address;
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
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001010;
            let t2 = ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000101;
            t1 || t2
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001010;
            if t1 {
                let rn_no = ins.hdr.idx(0, 3) as usize;
                let rm_no = ins.hdr.idx(3, 3) as usize;
                let (result, carry, over) = cortex_sub(cpu.r[rn_no], cpu.r[rm_no]);
                cpu.n = 0 < (result & (1 << 31));
                cpu.z = result == 0;
                cpu.c = carry;
                cpu.v = over;
            }
            if !t1 {
                let rn_no = ins.hdr.idx(0, 3) as usize;
                let rm_no = ins.hdr.idx(3, 4) as usize;
                let (result, carry, over) = cortex_sub(cpu.r[rn_no], cpu.r[rm_no]);
                cpu.n = 0 < (result & (1 << 31));
                cpu.z = result == 0;
                cpu.c = carry;
                cpu.v = over;
            }
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
        |ins, _cpu, _| {
            let _ext = ins.ext.unwrap();
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
        |ins, _cpu, _| {
            let _ext = ins.ext.unwrap();
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
        |ins, _cpu, _| {
            let _ext = ins.ext.unwrap();
            unimplemented!("Instruction sync barrier unimplmeneted")
        }
    );

    instructions.implement(
        "LDM, LMIA, LDMFD",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b11001,
        |ins, cpu, addresses| {
            let rn_no = ins.hdr.idx(8, 3) as usize;
            let reglist = ins.hdr.idx(0, 8) as AHalfWord; // aka bitmask
            // Load multiple registers accordin to bitmask starting at [rn_no]
            for i in 0..8 {
                if 0 == reglist.idx(i, 1) { continue; }
                cpu.r[i] = addresses.read_w(cpu.r[rn_no]);
                cpu.r[rn_no] += 4;
            }
        }
    );

    instructions.implement(
        "LDR (immediate)",
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b01101;
            let t2 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b10011;
            t1 || t2
        },
        |ins, cpu, addresses| {
            let t1 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b01101;
            if t1 {
                let rt_no = ins.hdr.idx(0, 3) as usize;
                let rn_no = ins.hdr.idx(3, 3) as usize;
                let imd = ins.hdr.idx(6, 5) as AWord;
                cpu.r[rt_no] = addresses.read_w(cpu.r[rn_no].wrapping_add(imd << 2));
            }
            if !t1 {
                let imd = ins.hdr.idx(0, 8) as AWord;
                let rt_no = ins.hdr.idx(8, 3) as usize;
                cpu.r[rt_no] = addresses.read_w(cpu.r[SP_IDX].wrapping_add(imd << 2));
            }
        }
    );

    instructions.implement(
        "LDR (literal)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01001,
        |ins, cpu, addresses| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rt_no = ins.hdr.idx(8, 3) as usize;
            let pc_read = cpu.r[PC_IDX] & !3; // clear 2 least sig bits
            let address = pc_read.wrapping_add(imd << 2);
            cpu.r[rt_no] = addresses.read_w(address);
        }
    );

    instructions.implement(
        "LDR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101100,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            cpu.r[rt_no] = addresses.read_w(cpu.r[rn_no] + (cpu.r[rm_no] << 2));
        }
    );

    instructions.implement(
        "LDRB (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01111,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            cpu.r[rt_no] = addresses.readb(cpu.r[rn_no] + (imd << 2)) as AWord;
        }
    );

    instructions.implement(
        "LDRB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101110,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            cpu.r[rt_no] = addresses.readb(cpu.r[rn_no] + (cpu.r[rm_no] << 2)) as AWord;
        }
    );

    instructions.implement(
        "LDRH (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10001,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            cpu.r[rt_no] = addresses.read_hw(cpu.r[rn_no] + (imd << 2)) as AWord;
        }
    );

    instructions.implement(
        "LDRSB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101011,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            cpu.r[rt_no] = addresses.readb(cpu.r[rn_no] + (cpu.r[rm_no] << 2)) as i8 as i32 as AWord;
        }
    );

    instructions.implement(
        "LDRSH (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101111,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            cpu.r[rt_no] = addresses.read_hw(cpu.r[rn_no] + (cpu.r[rm_no] << 2)) as i16 as i32 as u32;
        }
    );

    instructions.implement(
        "LSL (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00000,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            if 0 < imd {
                cpu.r[rd_no] = cpu.r[rm_no] << (imd - 1);
                cpu.c = bitidx(cpu.r[rm_no], 31, 1) > 0;
                cpu.r[rd_no] = cpu.r[rd_no] << 1;
            }
            cpu.z = cpu.r[rd_no] == 0;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
        }
    );

    instructions.implement(
        "LSL (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000010,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            if 0 < cpu.r[rm_no] {
                cpu.r[rdn_no] = cpu.r[rdn_no] << (cpu.r[rm_no] - 1);
                cpu.c = bitidx(cpu.r[rdn_no], 31, 1) > 0;
                cpu.r[rdn_no] = cpu.r[rdn_no] << 1;
            }
            cpu.z = cpu.r[rdn_no] == 0;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
        }
    );

    instructions.implement(
        "LSR (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00001,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            if 0 < imd {
                cpu.r[rd_no] = cpu.r[rm_no] >> (imd - 1);
                cpu.c = bitidx(cpu.r[rd_no], 31, 1) > 0;
                cpu.r[rd_no] = cpu.r[rd_no] >> 1;
            }
            cpu.z = cpu.r[rd_no] == 0;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
        }
    );

    instructions.implement(
        "LSR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000011,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            if 0 < cpu.r[rm_no] {
                cpu.r[rdn_no] = cpu.r[rm_no] >> (cpu.r[rm_no] - 1);
                cpu.c = bitidx(cpu.r[rdn_no], 31, 1) > 0;
                cpu.r[rdn_no] = cpu.r[rdn_no] >> 1;
            }
            cpu.z = cpu.r[rdn_no] == 0;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
        }
    );

    instructions.implement(
        "MOV (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b00100,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 8) as AWord;
            let rd_no = ins.hdr.idx(8, 3) as usize;
            cpu.r[rd_no] = imd;
            cpu.z = cpu.r[rd_no] == 0;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
        }
    );

    instructions.implement(
        "MOV (register)",
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000110;
            let t2 = ins.is_t1() && ins.hdr.idx(6, 10) == 0b0000000000;
            t1 || t2
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && ins.hdr.idx(8, 8) == 0b01000110;
            if t1 {
                let rd_no = ins.hdr.idx(0, 3) as usize;
                let rm_no = ins.hdr.idx(3, 4) as usize;
                cpu.r[rd_no] = cpu.r[rm_no];
                cpu.z = cpu.r[rd_no] == 0;
                cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            }
            if !t1 {
                let rd_no = ins.hdr.idx(0, 3) as usize;
                let rm_no = ins.hdr.idx(3, 3) as usize;
                cpu.r[rd_no] = cpu.r[rm_no];
                cpu.z = cpu.r[rd_no] == 0;
                cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            }
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
        |ins, _cpu, _| {
            let _ext = ins.ext.unwrap();
            unimplemented!()
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
        |ins, _cpu, _| {
            let _ext = ins.ext.unwrap();

            unimplemented!()
        }
    );

    instructions.implement(
        "MUL",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001101,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] *= cpu.r[rm_no];

            cpu.z = cpu.r[rdn_no] == 0;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
        }
    );

    instructions.implement(
        "MVN",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001101,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = !cpu.r[rn_no];

            cpu.z = cpu.r[rd_no] == 0;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
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
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] |= cpu.r[rm_no];

            cpu.z = cpu.r[rdn_no] == 0;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
        }
    );

    instructions.implement(
        "POP",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b1011110,
        |ins, cpu, addresses| {
            let p = ins.hdr.idx(8, 1) > 0;
            let reglist = ins.hdr.idx(0, 8) as AHalfWord; // aka bitmask
            
            if p {
                cpu.r[PC_IDX] = addresses.read_w(cpu.r[SP_IDX]);
                cpu.r[SP_IDX] += 4;
            }
            for i in 0..8 {
                let j = 7 - i;
                if 0 == reglist.idx(j, 1) { continue; }
                cpu.r[j] = addresses.read_w(cpu.r[SP_IDX]);
                cpu.r[SP_IDX] += 4;
            }
            //exit(1);
        }
    );

    instructions.implement(
        "PUSH",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b1011010,
        |ins, cpu, addresses| {
            let m = ins.hdr.idx(8, 1) > 0;
            let reglist = ins.hdr.idx(0, 8) as AHalfWord; // aka bitmask
            
            for i in 0..8 {
                if 0 == reglist.idx(i, 1) { continue; }
                cpu.r[SP_IDX] -= 4;
                addresses.write_w(cpu.r[SP_IDX], cpu.r[i]);
            }
            if m {
                cpu.r[SP_IDX] -= 4;
                addresses.write_w(cpu.r[SP_IDX], cpu.r[LR_IDX]);
            }
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
            cpu.r[rd_no] = (cpu.r[rn_no] as AHalfWord).swap_bytes() as i16 as i32 as AWord;
        }
    );

    instructions.implement(
        "ROR",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000111,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rdn_no] = cpu.r[rdn_no].rotate_right(cpu.r[rm_no]);

            cpu.z = cpu.r[rdn_no] == 0;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.c = 0 < (cpu.r[rdn_no] & (1 << 31));
        }
    );

    instructions.implement(
        "RSB (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001001,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let (result, carry, over) = cortex_sub(0, cpu.r[rn_no]);
            cpu.r[rd_no] = result;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
            cpu.v = over;

        }
    );

    instructions.implement(
        "SBC (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100000110,
        |ins, cpu, _| {
            let rdn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            //Kind of the exact opposite of adc: Rd = Rn - Rm - (1 - C) 
            let (result1, carry1, over1) = cortex_sub(cpu.r[rdn_no], cpu.r[rm_no]);
            let (result2, carry2, over2) = cortex_sub(result1, if cpu.c {1} else {0});
            cpu.r[rdn_no] = result2;
            cpu.c = carry1 || carry2;
            cpu.v = over1 || over2;
            cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
            cpu.z = cpu.r[rdn_no] == 0;
        }
    );

    instructions.implement(
        "SEV",
        |ins| ins.is_t1() && ins.hdr == 0b1011111101000000,
        |_, _, _| {
            unimplemented!()
        }
    );

    instructions.implement(
        "STM, STMIA, STMEA",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b11000,
        |ins, cpu, addresses| {
            let rn_no = ins.hdr.idx(8, 3) as usize;
            let reglist = ins.hdr.idx(0, 8) as AHalfWord;
            // Store multiple registers according to bitmask starting at [rn_no]
            for i in 0..8 {
                if 0 == reglist.idx(i, 1) { continue; }
                addresses.write_w(cpu.r[rn_no], cpu.r[i]);
                cpu.r[rn_no] += 4;
            }
        }
    );

    instructions.implement(
        "STR (immediate)",
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b01100;
            let t2 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b10010;
            t1 || t2
        },
        |ins, cpu, addresses| {
            let t1 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b01100;
            if t1 {
                let rt_no = ins.hdr.idx(0, 3) as usize;
                let rn_no = ins.hdr.idx(3, 3) as usize;
                let imd = ins.hdr.idx(6, 5) as AWord;
                let address = cpu.r[rn_no].wrapping_add(imd << 2);
                addresses.write_w(address, cpu.r[rt_no]);
            }
            if !t1 {
                let imd = ins.hdr.idx(0, 8) as AWord;
                let rt_no = ins.hdr.idx(8, 3) as usize;
                addresses.write_w(cpu.r[SP_IDX].wrapping_add(imd << 2), cpu.r[rt_no]);
            }
        }
    );

    instructions.implement(
        "STR (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101000,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            addresses.write_w(cpu.r[rn_no]+cpu.r[rm_no], cpu.r[rt_no]);
        }
    );

    instructions.implement(
        "STRB (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b01110,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            addresses.writeb(cpu.r[rn_no]+imd, cpu.r[rt_no] as AByte);
        }
    );

    instructions.implement(
        "STRB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101010,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            addresses.writeb(cpu.r[rn_no]+cpu.r[rm_no], cpu.r[rt_no] as AByte);
        }
    );

    instructions.implement(
        "STRH (immediate)",
        |ins| ins.is_t1() && ins.hdr.idx(11, 5) == 0b10001,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let imd = ins.hdr.idx(6, 5) as AWord;
            addresses.write_hw(cpu.r[rn_no]+imd, cpu.r[rt_no] as AHalfWord);
        }
    );

    instructions.implement(
        "STRH (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0101001,
        |ins, cpu, addresses| {
            let rt_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            addresses.write_hw(cpu.r[rn_no]+cpu.r[rm_no], cpu.r[rt_no] as AHalfWord);
        }
    );

    instructions.implement(
        "SUB (immediate)",
        |ins| {
            let t1 = ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001111;
            let t2 = ins.is_t1() && ins.hdr.idx(11, 5) == 0b00111;
            t1 || t2
        },
        |ins, cpu, _| {
            let t1 = ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001111;
            if t1 {
                let rd_no = ins.hdr.idx(0, 3) as usize;
                let rn_no = ins.hdr.idx(3, 3) as usize;
                let imd = ins.hdr.idx(6, 3) as AWord;
                let (result, carry, over) = cortex_sub(cpu.r[rn_no], imd);
                cpu.r[rd_no] = result;
                cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
                cpu.z = cpu.r[rd_no] == 0;
                cpu.c = carry;
                cpu.v = over;
            }
            if !t1 {
                let imd = ins.hdr.idx(0, 8) as AWord;
                let rdn_no = ins.hdr.idx(8, 3) as usize;
                let (result, carry, over) = cortex_sub(cpu.r[rdn_no], imd);
                cpu.r[rdn_no] = result;
                cpu.n = 0 < (cpu.r[rdn_no] & (1 << 31));
                cpu.z = cpu.r[rdn_no] == 0;
                cpu.c = carry;
                cpu.v = over;
            }
        }
    );

    instructions.implement(
        "SUB (register)",
        |ins| ins.is_t1() && ins.hdr.idx(9, 7) == 0b0001101,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rn_no = ins.hdr.idx(3, 3) as usize;
            let rm_no = ins.hdr.idx(6, 3) as usize;
            let (result, carry, over) = cortex_sub(cpu.r[rn_no], cpu.r[rm_no]);
            cpu.r[rd_no] = result;
            cpu.n = 0 < (cpu.r[rd_no] & (1 << 31));
            cpu.z = cpu.r[rd_no] == 0;
            cpu.c = carry;
            cpu.v = over;
        }
    );

    instructions.implement(
        "SUB (SP minus intermediate)",
        |ins| ins.is_t1() && ins.hdr.idx(7, 9) == 0b10110001,
        |ins, cpu, _| {
            let imd = ins.hdr.idx(0, 7) as AWord;
            cpu.r[SP_IDX] -= imd << 2;
        }
    );

    instructions.implement(
        "SVC",
        |ins| ins.is_t1() && ins.hdr.idx(8, 8) == 0b11011111,
        |ins, _cpu, _| {
            let _imd = ins.hdr.idx(0, 7) as AWord;
            unimplemented!()
        }
    );

    instructions.implement(
        "SXTB",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001001,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = cpu.r[rm_no] as AByte as i8 as i32 as AWord;
        }
    );

    instructions.implement(
        "SXTH",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001000,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = cpu.r[rm_no] as AHalfWord as i16 as i32 as AWord;
        }
    );

    instructions.implement(
        "TST (register)",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b0100001000,
        |ins, cpu, _| {
            let rn_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            let result = cpu.r[rn_no] & cpu.r[rm_no];
            cpu.n = 0 < (result & (1 << 31));
            cpu.z = result == 0;
        }
    );

    instructions.implement(
        "UDF",
        |ins| {
            // TODO: Has alternate t2 encoding
            ins.is_t1() && ins.hdr.idx(8, 8) == 0b11011110
        },
        |ins, _cpu, _| {
            let _imd = ins.hdr.idx(0, 8) as usize;
            // undefined instruction exception generation
            unimplemented!()
        }
    );

    instructions.implement(
        "UXTB",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001011,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = cpu.r[rm_no] as u8 as u32;
        }
    );

    instructions.implement(
        "UXTH",
        |ins| ins.is_t1() && ins.hdr.idx(6, 10) == 0b1011001010,
        |ins, cpu, _| {
            let rd_no = ins.hdr.idx(0, 3) as usize;
            let rm_no = ins.hdr.idx(3, 3) as usize;
            cpu.r[rd_no] = cpu.r[rm_no] as u16 as u32;
        }
    );

    instructions.implement(
        "WFE",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100100000,
        |_ins, _cpu, _| {
            unimplemented!()
        }
    );

    instructions.implement(
        "WFI",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100110000,
        |_ins, _cpu, _| {
            unimplemented!()
        }
    );

    instructions.implement(
        "YIELD",
        |ins| ins.is_t1() && ins.hdr == 0b1011111100010000,
        |_ins, _cpu, _| {
            unimplemented!()
        }
    );

}
