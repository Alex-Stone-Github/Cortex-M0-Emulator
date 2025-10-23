use std::ops::DerefMut;
use crate::adr::AddressSpace;
use crate::core::*;
pub struct AddressDeMultiplexer<'a> {
    origin: AWord,
    length: AWord,
    regions: Vec<Box<dyn AddressSpace + 'a>>,
}
impl<'a> AddressDeMultiplexer<'a> {
    pub fn full() -> Self {
        return Self::new(0, AWord::MAX)
    }
    pub fn new(origin: AWord, length: AWord) -> Self {
        Self {origin, length, regions: Vec::new()}
    }
    fn lookup(&mut self, idx: AWord) -> Option<(&mut dyn AddressSpace, AWord)> {
        for region in self.regions.iter_mut() {
            let contains_idx = region.origin() <= idx && idx < region.origin() + region.len();
            if contains_idx {
                let local_idx = idx - region.origin();
                return Some((region.deref_mut(), local_idx));
            }
        }
        None
    }
    pub fn add_region(&mut self, region: Box<dyn AddressSpace + 'a>) {
        self.regions.push(region);
    }
}
impl<'a> AddressSpace for AddressDeMultiplexer<'a> {
    fn origin(&self) -> AWord {self.origin}
    fn len(&self) -> AWord {self.length}
    fn readb(&mut self, adr: AWord) -> AByte {
        let (region, lidx) = self.lookup(adr).expect("SEGFAULT");
        region.readb(lidx)
    }
    fn writeb(&mut self, adr: AWord, x: AByte) {
        let (region, lidx) = self.lookup(adr).expect("SEGFAULT");
        region.writeb(lidx, x);
    }
}

// Basically the dumbest we can get
#[derive(Debug)]
pub struct BufferMemory {
    pub origin: AWord,
    pub buffer: Box<[u8]>,
}

impl AddressSpace for BufferMemory {
    fn readb(&mut self, adr: AWord) -> AByte {
        return self.buffer[adr as usize];
    }
    fn writeb(&mut self, adr: AWord, x: AByte) {
        self.buffer[adr as usize] = x;
    }
    fn origin(&self) -> AWord {self.origin}
    fn len(&self) -> AWord {self.buffer.len() as AWord}
}
pub struct FunctionalAddressSpace {
    pub origin: AWord,
    pub length: AWord,
    pub readb_f: Box<dyn FnMut(AWord) -> AByte>,
    pub writeb_f: Box<dyn FnMut(AWord, AByte)>,
}
impl AddressSpace for FunctionalAddressSpace {
    fn origin(&self) -> AWord {self.origin}
    fn len(&self) -> AWord {self.length}
    fn readb(&mut self, adr: AWord) -> AByte {self.readb_f.deref_mut()(adr)}
    fn writeb(&mut self, adr: AWord, x: AByte) {self.writeb_f.deref_mut()(adr, x)}
}
#[test]
fn test_lsb_read() {
    let mem = [3, 0];
    let mut sample_adr = BufferMemory{
        origin: 0,
        buffer: Box::new(mem)
        };
    let info = sample_adr.read_hw_le(0);
    assert_eq!(info, 3);
}
#[test]
fn test_lookup() {
    let mem = [3, 69];
    let sample_adr = BufferMemory{
        origin: 2,
        buffer: Box::new(mem)
        };
    let mut de = AddressDeMultiplexer::full();
    de.add_region(Box::new(sample_adr));

    assert!(de.lookup(0).is_none());
    assert_eq!(de.lookup(3).unwrap().1, 1);
    assert_eq!(de.lookup(3).unwrap().0.readb(1), 69);
}

#[test]
fn test_func_adr() {
    let mut fa = FunctionalAddressSpace {
        origin: 0,
        length: 100,
        readb_f: Box::new(|a| a as AByte),
        writeb_f: Box::new(|_, _| {})
    };

    assert_eq!(fa.readb(0), 0);
    assert_eq!(fa.readb(69), 69);
}
