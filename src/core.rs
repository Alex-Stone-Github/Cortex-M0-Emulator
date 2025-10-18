pub type AByte = u8;
pub type AHalfWord = u16;
pub type AWord = u32;

pub fn bitidx<I>(x: I, ptr: usize, len: usize) -> I where 
I : std::ops::Shl<usize, Output = I> +
std::ops::Shr<usize, Output = I> + 
Copy {
    let bitcount = std::mem::size_of::<I>() * 8;
    let move_dist = bitcount - len - ptr;
    let topped = x << move_dist;
    let bottomed = topped >> (ptr + move_dist);
    bottomed
}

#[test]
fn test_bitidx() {
    let byte: u8 = 0b11110000;
    assert_eq!(bitidx(byte, 4, 4), 0b1111);
    assert_eq!(bitidx(byte, 0, 4), 0b0000);
    assert_eq!(bitidx(byte, 2, 4), 0b1100);

    let half_word: u16 = 0b1010101011110000;
    assert_eq!(bitidx(half_word, 4, 4), 0b1111);
    assert_eq!(bitidx(half_word, 0, 4), 0b0000);
    assert_eq!(bitidx(half_word, 2, 4), 0b1100);
}
