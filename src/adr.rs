use crate::core::{AByte, AHalfWord, AWord};

pub const SAMPLE: [AByte; 48] = [
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


const LITTLE_ENDIAN: bool = true;

pub trait AddressSpace {
    fn readb(&mut self, adr: AWord) -> AByte;
    fn writeb(&mut self, adr: AWord, x: AByte);

    // Reads
    fn read_hw_le(&mut self, adr: AWord) -> AHalfWord {
        debug_assert!(adr % 2 == 0);
        let bytes: [AByte; 2] = [
            self.readb(adr),
            self.readb(adr+1)
        ];
        unsafe { std::mem::transmute::<[AByte; 2], AHalfWord>(bytes) }
    }
    fn read_hw_be(&mut self, adr: AWord) -> AHalfWord {self.read_hw_le(adr).swap_bytes()}
    fn read_w_le(&mut self, adr: AWord) -> AWord {
        debug_assert!(adr % 4 == 0);
        let bytes: [AByte; 4] = [
            self.readb(adr),
            self.readb(adr+1),
            self.readb(adr+2),
            self.readb(adr+3),
        ];
        // Probably defined(same size at least)
        unsafe { std::mem::transmute::<[AByte; 4], AWord>(bytes) }
    }
    fn read_w_be(&mut self, adr: AWord) -> AWord {self.read_w_le(adr).swap_bytes()}

    // Writes
    fn write_hw_le(&mut self, adr: AWord, x: AHalfWord) {
        debug_assert!(adr % 2 == 0);
        let bytes = unsafe { std::mem::transmute::<AHalfWord, [AByte; 2]>(x) };
        self.writeb(adr, bytes[0]);
        self.writeb(adr + 1, bytes[1]);
    }
    fn write_hw_be(&mut self, adr: AWord, x: AHalfWord) {self.write_hw_le(adr, x.swap_bytes());}
    fn write_w_le(&mut self, adr: AWord, x: AWord) {
        debug_assert!(adr % 2 == 0);
        let bytes = unsafe { std::mem::transmute::<AWord, [AByte; 4]>(x) };
        self.writeb(adr, bytes[0]);
        self.writeb(adr + 1, bytes[1]);
        self.writeb(adr + 2, bytes[2]);
        self.writeb(adr + 3, bytes[3]);
    }
    fn write_w_be(&mut self, adr: AWord, x: AWord) {self.write_w_le(adr, x.swap_bytes());}

    // Aliases
    fn read_hw(&mut self, adr: AWord) -> AHalfWord {
        match LITTLE_ENDIAN {
            true => self.read_hw_le(adr),
            false => self.read_hw_be(adr)
        }
    }
    fn read_w(&mut self, adr: AWord) -> AWord {
        match LITTLE_ENDIAN {
            true => self.read_w_le(adr),
            false => self.read_w_be(adr)
        }
    }
    fn write_hw(&mut self, adr: AWord, x: AHalfWord) {
        match LITTLE_ENDIAN {
            true => self.write_hw_le(adr, x),
            false => self.write_hw_be(adr, x)
        }
    }
    fn write_w(&mut self, adr: AWord, x: AWord) {
        match LITTLE_ENDIAN {
            true => self.write_w_le(adr, x),
            false => self.write_w_be(adr, x)
        }
    }
}

#[test]
fn test_lsb_read() {
    let mut mem = [3, 0];
    let mut sample_adr = Sample(&mut mem);
    let info = sample_adr.read_hw_le(0);
    assert_eq!(info, 3);
}

pub struct Sample<'a>(pub &'a [u8]);

impl<'a> AddressSpace for Sample<'a> {
    fn readb(&mut self, adr: AWord) -> AByte {
        return self.0[adr as usize];
    }
    fn writeb(&mut self, adr: AWord, x: AByte) {
        unimplemented!()
    }
}
