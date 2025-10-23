use crate::adr::AddressSpace;
use crate::core::*;
use crate::ins::*;

/// Load a new instruction, either thumb1 or thumb2, appropriately incrementing ip
pub fn fetch_instruction(ip: &mut AWord, memory: &mut dyn AddressSpace) -> InsData {
    let mut load_half_word = || -> AHalfWord {
        // Load Instruction
        let load_adr = ip.wrapping_sub(2);
        let half_word = memory.read_hw_le(load_adr);
        // Advance to next instruction
        *ip = ip.wrapping_add(2);
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
    use crate::memory::BufferMemory;
    let mut ip = 4;
    let mut cont = [0; 4];
    cont[0] = 0b00111111; // Some sort of shift
    let mut memory = BufferMemory{
        origin: 0,
        buffer: Box::new(cont),
    };
    let gimmi = fetch_instruction(&mut ip, &mut memory);
    assert!(gimmi.is_t1());
}
