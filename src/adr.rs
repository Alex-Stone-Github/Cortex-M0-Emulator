use crate::core::{AByte, AHalfWord, AWord};

const LITTLE_ENDIAN: bool = true;

pub trait AddressSpace {
    fn origin(&self) -> AWord;
    fn len(&self) -> AWord;

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

