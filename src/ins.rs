use crate::core::*;

/// Instruction is stored as a u16, such that the first byte loaded is the most significant byte of
/// the u16
#[derive(Debug, Clone)]
pub enum Instruction {
    Thumb1(AHalfWord),
    Thumb2(AHalfWord, AHalfWord),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionClass {
    ShiftAddMoveCompare,
    DataProcessing,
    SpecialDataBranchExchange,
    LoadLiteral,
    LoadStore,
    GeneratePCAdr,
    GenerateSPAdr,
    Misc,
    StoreRegs,
    LoadRegs,
    CondBranch,
    UnCondBranch,
}
impl Instruction {
    /// Get the opcode of the instruction
    pub fn opcode(&self) -> u8 {
        let first_half_word = match self {
            Self::Thumb1(instruction) => instruction,
            Self::Thumb2(instruction, _) => instruction,
        };
        let opcode = first_half_word >> (16 - 6); // get first 6 bits
        return opcode as u8;
    }
    /// Print the classification of an instruction
    pub fn get_class(&self) -> Option<InstructionClass> {
        let code = self.opcode();
        match code {
            x if 0 == (x >> 4) => Some(InstructionClass::ShiftAddMoveCompare),
            x if 0b010000 == x => Some(InstructionClass::DataProcessing),
            x if 0b010001 == x => Some(InstructionClass::SpecialDataBranchExchange),
            x if 0b01001 == (x >> 1) => Some(InstructionClass::LoadLiteral),
            x if 0b0101 == (x >> 2) || 0b011 == (x >> 3) || 0b100 == (x >> 3) => Some(InstructionClass::LoadStore),
            x if 0b10100 == (x >> 1) => Some(InstructionClass::GeneratePCAdr),
            x if 0b10101 == (x >> 1) => Some(InstructionClass::GenerateSPAdr),
            x if 0b1011 == (x >> 2) => Some(InstructionClass::Misc),
            x if 0b11000 == (x >> 1) => Some(InstructionClass::StoreRegs),
            x if 0b11001 == (x >> 1) => Some(InstructionClass::LoadRegs),
            x if 0b1101 == (x >> 2) => Some(InstructionClass::CondBranch),
            x if 0b11100 == (x >> 1) => Some(InstructionClass::UnCondBranch),
            _ => None
        }
    }
}

/// Load a new instruction, either thumb1 or thumb2, appropriately incrementing ip
pub fn fetch_instruction(ip: &mut AWord, memory: &[AByte]) -> Instruction {
    let mut load_half_word = || -> AHalfWord {
        let least_significant_byte = memory[*ip as usize] as AHalfWord;
        let most_significant_byte = memory[*ip as usize + 1] as AHalfWord;
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
        false => Instruction::Thumb1(instruction),
        true => {
            let extended_instruction_part = load_half_word();
            Instruction::Thumb2(instruction, extended_instruction_part)
        },
    }
}

#[test]
fn test_instruction_load() {
    let mut ip = 0;
    let mut memory = [0; 4];
    memory[0] = 0b00111111; // Some sort of shift
    let gimmi = fetch_instruction(&mut ip, &memory);
    assert_eq!(gimmi.opcode(), 0b001111);
    assert!(matches!(gimmi, Instruction::Thumb1(_)));
    assert!(matches!(gimmi.get_class(), Some(InstructionClass::ShiftAddMoveCompare)));
    assert!(!matches!(gimmi.get_class(), Some(InstructionClass::DataProcessing)));
}
#[test]
fn test_other_ins() {
    let mut ip = 0;
    let mut memory = [0; 4];
    memory[0] = 0b10101000; // Some sort of shift
    let gimmi = fetch_instruction(&mut ip, &memory);
    assert_eq!(gimmi.opcode(), 0b101010);
    assert!(matches!(gimmi, Instruction::Thumb1(_)));
    assert!(matches!(gimmi.get_class(), Some(InstructionClass::GenerateSPAdr)));
    assert!(!matches!(gimmi.get_class(), Some(InstructionClass::GeneratePCAdr)));
}
#[test]
fn test_load_all_classes() {
    let classes = [
        Some(InstructionClass::ShiftAddMoveCompare),
        Some(InstructionClass::DataProcessing),
        Some(InstructionClass::SpecialDataBranchExchange),
        Some(InstructionClass::LoadLiteral),
        Some(InstructionClass::LoadStore),
        Some(InstructionClass::LoadStore),
        Some(InstructionClass::LoadStore),
        Some(InstructionClass::GeneratePCAdr),
        Some(InstructionClass::GenerateSPAdr),
        Some(InstructionClass::Misc),
        Some(InstructionClass::StoreRegs),
        Some(InstructionClass::LoadRegs),
        Some(InstructionClass::CondBranch),
        Some(InstructionClass::UnCondBranch),
    ];
    let memory = [
        0, 0,          // Shift
        0b01000000, 0, // Data Processing
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
    let size = memory.len() / 2 as usize;
    let mut ip = 0;
    for idx in 0..size {
        let ins = fetch_instruction(&mut ip, &memory);
        assert_eq!(ins.get_class(), classes[idx])
    }
}
