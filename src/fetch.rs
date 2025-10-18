use crate::adr::AddressSpace;
use crate::adr::Sample;
use crate::core::*;
use crate::ins::*;

/// Load a new instruction, either thumb1 or thumb2, appropriately incrementing ip
pub fn fetch_instruction(ip: &mut AWord, memory: &mut dyn AddressSpace) -> InsData {
    let mut load_half_word = || -> AHalfWord {
        let least_significant_byte = memory.readb(*ip) as AHalfWord;
        let most_significant_byte = memory.readb(*ip + 1) as AHalfWord;
        let mut half_word = 0;
        half_word = half_word | (least_significant_byte << 8);
        half_word = half_word | (most_significant_byte << 0);
        *ip += 2;
        return half_word;
    };
    let instruction = load_half_word();
    let load_another_byte_clue = instruction >> (16-5);
    let load_extended_32_instruction = match load_another_byte_clue {
        0b11101 |
        0b11110 |
        0b11111 => true,
        _ => false,
    };
    match load_extended_32_instruction {
        false => InsData{ hdr: instruction, ext: None },
        true => {
            let extended_instruction_part = load_half_word();
            InsData{
                hdr: instruction,
                ext: Some(extended_instruction_part),
            }
        },
    }
}

#[test]
fn test_instruction_load() {
    let mut ip = 0;
    let mut cont = [0; 4];
    cont[0] = 0b00111111; // Some sort of shift
                            //
    let mut memory = Sample(&cont);
    let gimmi = fetch_instruction(&mut ip, &mut memory);
    assert!(gimmi.is_t1());
}
