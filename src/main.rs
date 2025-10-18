mod core;
mod ins;
mod registers;

fn main() {
    println!("Hello, world!");

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
    let cpu = &mut registers::Registers {
        R: [0; 13],
        SP: 0,
        LR: 0,
        PC: 0,
    };
    registers::execute(cpu, &memory);
}
