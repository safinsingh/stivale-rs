/// The framebuffer info passed by the bootloader
/// and based on the configuration in the stivale2 header
#[repr(packed)]
pub struct FramebufferTag {
    _identifier: u64,
    _next: u64,
    address: u64,
    width: u16,
    height: u16,
    pitch: u16,
    bpp: u16,
}

impl FramebufferTag {
    /// Get the start address of the framebuffer
    pub const fn start_address(&self) -> usize {
        self.address as usize
    }

    /// Get the end address of the framebuffer
    /// 
    /// Identical to `framebuffer_info.start_address() + framebuffer_info.size()`
    pub const fn end_address(&self) -> usize {
        self.address as usize + self.size()
    }

    /// Get the size of the framebuffer
    pub const fn size(&self) -> usize {
        self.pitch as usize * self.height as usize * (self.bpp as usize / 8)
    }

    /// Get the width of the framebuffer in pixels
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Get the height of the framebuffer in pixels
    pub const fn height(&self) -> u16 {
        self.height
    }

    /// Get the bytes per line of the framebuffer
    pub const fn pitch(&self) -> u16 {
        self.pitch
    }

    /// Get the bits per pixel of the framebuffer
    pub const fn bpp(&self) -> u16 {
        self.bpp
    }
}
