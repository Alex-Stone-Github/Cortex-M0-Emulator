use crate::{adr::AddressSpace, core::*, registers};

/// Instruction is stored as a u16, such that the first byte loaded is the most significant byte of
/// the u16
#[derive(Debug, Clone)]
pub struct InsData {
    pub hdr: AHalfWord,
    pub ext: Option<AHalfWord>
}
impl InsData {
    pub fn is_t1(&self) -> bool {
        self.ext.is_none()
    }
}
pub struct InsType {
    pub name: &'static str,
    pub is_me: fn(&InsData) -> bool,
    pub execute: fn(&InsData, &mut registers::Registers, &mut dyn AddressSpace)
}

pub struct LoaderExecuter {
    pub instruction_types: Vec<InsType>,
}
impl LoaderExecuter {
    pub fn new() -> Self {
        Self {
            instruction_types: Vec::new(),
        }
    }
    pub fn implement(&mut self,
        name: &'static str,
        is_me: fn(&InsData) -> bool, 
        execute: fn(&InsData, &mut registers::Registers, &mut dyn AddressSpace)
            ) {
        let ins_type = InsType{name, is_me, execute};
        self.instruction_types.push(ins_type);
    }
    pub fn execute(&self, instruction: &InsData, regs: &mut registers::Registers, memory: &mut dyn AddressSpace) {
        let instruction_type = self.instruction_types.iter().find(|ins| (ins.is_me)(instruction))
            .unwrap_or(&InsType { name: "_INVALID(Skipped)", is_me: |_|true, execute: |_, _, _| {}});
        (instruction_type.execute)(instruction, regs, memory);
        println!("Executed `{:<20}` HDR: {:016b}!", instruction_type.name, instruction.hdr);
    }
}

