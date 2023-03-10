#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FrameBufferConfig {
    pub frame_buffer_base_addr: u64,
    pub frame_buffer_size: usize,
    pub pixel_per_scanline: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
    pub pixel_format: PixelFormat,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

impl FrameBufferConfig {
    pub fn new(
        frame_buffer_base: u64,
        frame_buffer_size: usize,
        pixel_per_scanline: usize,
        horizontal_resolution: usize,
        vertical_resolution: usize,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            frame_buffer_base_addr: frame_buffer_base,
            frame_buffer_size,
            pixel_per_scanline,
            horizontal_resolution,
            vertical_resolution,
            pixel_format,
        }
    }

    pub fn mock() -> Self {
        Self {
            frame_buffer_base_addr: 0,
            frame_buffer_size: 0,
            pixel_per_scanline: 0,
            horizontal_resolution: 0,
            vertical_resolution: 0,
            pixel_format: PixelFormat::Rgb,
        }
    }
    pub fn frame_buffer_base_ptr(&self) -> *mut u8 {
        self.frame_buffer_base_addr as *mut u8
    }
}
