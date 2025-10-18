mod core;
mod ins;
mod registers;

fn main() {
    println!("Hello, world!");

    let memory = [
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
