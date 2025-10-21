use std::ops::DerefMut;

use crate::adr::AddressSpace;
use crate::core::*;

pub struct AddressDeMultiplexer<'a> {
    regions: Vec<Box<dyn AddressSpace + 'a>>,
}
impl<'a> AddressDeMultiplexer<'a> {
    pub fn new() -> Self {
        Self {regions: Vec::new()}
    }
    fn lookup(&mut self, idx: AWord) -> Option<(&mut dyn AddressSpace, AWord)> {
        for region in self.regions.iter_mut() {
            let contains_idx = region.origin() < idx && idx < region.origin() + region.len();
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

// Basically the dumbest we can get
#[derive(Debug)]
pub struct BufferMemory<'a>{
    pub origin: AWord,
    pub buffer: &'a mut [u8],
}

impl<'a> AddressSpace for BufferMemory<'a> {
    fn readb(&mut self, adr: AWord) -> AByte {
        return self.buffer[adr as usize];
    }
    fn writeb(&mut self, adr: AWord, x: AByte) {
        self.buffer[adr as usize] = x;
    }
    fn origin(&self) -> AWord {self.origin}
    fn len(&self) -> AWord {self.buffer.len() as AWord}
}
#[test]
fn test_lsb_read() {
    let mut mem = [3, 0];
    let mut sample_adr = BufferMemory{
        origin: 0,
        buffer: &mut mem
        };
    let info = sample_adr.read_hw_le(0);
    assert_eq!(info, 3);
}
#[test]
fn test_lookup() {
    let mut mem = [3, 69];
    let mut sample_adr = BufferMemory{
        origin: 2,
        buffer: &mut mem
        };
    let mut de = AddressDeMultiplexer::new();
    de.add_region(Box::new(sample_adr));

    assert!(de.lookup(0).is_none());
    assert_eq!(de.lookup(3).unwrap().1, 1);
    assert_eq!(de.lookup(3).unwrap().0.readb(1), 69);
}
