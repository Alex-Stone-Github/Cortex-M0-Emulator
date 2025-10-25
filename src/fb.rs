use crate::adr::AddressSpace;
use crate::core::*;

type Pixel = u32;

pub struct FramebufferDevice {
    pub origin: AWord,
    pub width: usize,
    pub height: usize,
    pub title: String,
    window: minifb::Window,
    buffer: Box<[u8]>
}
impl FramebufferDevice {
    pub fn new(origin: AWord, width: usize, height: usize, title: String) -> Self {
        let window = minifb::Window::new(&title, width, height, minifb::WindowOptions::default())
            .expect("Could not create fb device window");
        let buffer = vec![0; width * height * 4].into_boxed_slice();
        Self {
            origin,
            width,
            height,
            title,
            window,
            buffer,
        }

    }
    /// Returns if the window should close
    pub fn tick(&mut self) -> bool {
        let should_close = !self.window.is_open() || self.window.is_key_down(minifb::Key::Escape);

        if !should_close {
            // Should basically be like a reinterprete cast
            let buffer: &[u32] = unsafe {
                std::slice::from_raw_parts(
                    self.buffer.as_ptr() as *const u32,
                    self.width * self.height)
            };
            self.window.update_with_buffer(buffer, self.width, self.height)
                .expect("Window closed");
        }

        return should_close;
    }
}

impl AddressSpace for FramebufferDevice {
    fn origin(&self) -> AWord {self.origin}
    fn len(&self) -> AWord {(self.width * self.height * std::mem::size_of::<Pixel>()) as AWord}
    fn writeb(&mut self, adr: AWord, x: AByte) {
        self.buffer[adr as usize] = x;
        
    }
    fn readb(&mut self, _adr: AWord) -> AByte {
        log::warn!("Should not read from framebuffer device");
        return 0;
    }
}
