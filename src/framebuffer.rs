use std::os::fd::AsRawFd;
use std::path::PathBuf;

pub struct FramebufferConfig {
    pub path: PathBuf,
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
    pub stride_pixels: usize,
}

/// Represents a memory-mapped framebuffer.
pub struct Framebuffer {
    width: usize,
    height: usize,
    ptr: *mut u16,
    /// The number of pixels in a single row of the framebuffer.
    stride: usize,
}

impl Framebuffer {
    /// Creates a new [`Framebuffer`] mapped to the given path with the specified width and height.
    pub fn new(config: FramebufferConfig) -> anyhow::Result<Framebuffer> {
        // open framebuffer
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(config.path)?;

        let fd = file.as_raw_fd();

        let size = config.stride_pixels * config.height * config.bytes_per_pixel;

        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            )
        } as *mut u16;

        if ptr == libc::MAP_FAILED as *mut u16 {
            return Err(anyhow::anyhow!("Failed to mmap framebuffer"));
        }
        Ok(Framebuffer {
            width: config.width,
            height: config.height,
            ptr,
            stride: config.stride_pixels,
        })
    }

    pub fn write(&self, buf: &[u8]) {
        let src_w = crate::SCREEN_W as f32;
        let src_h = crate::SCREEN_H as f32;

        let dst_h = self.height as f32;

        // scale based on height
        let scale = dst_h / src_h; // 240 / 144 ≈ 1.6667

        let scaled_w = (src_w * scale).round() as usize;
        let x_offset = ((self.width - scaled_w) / 2) as isize;

        for sy in 0..crate::SCREEN_H {
            // y dest (float → int)
            let dy = (sy as f32 * scale).round() as isize;

            if dy < 0 || dy >= self.height as isize {
                continue;
            }

            for sx in 0..crate::SCREEN_W {
                let i = (sy * crate::SCREEN_W + sx) * 3;

                let r = buf[i];
                let g = buf[i + 1];
                let b = buf[i + 2];

                let rgb565: u16 =
                    ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3);

                let dx = (sx as f32 * scale).round() as isize + x_offset;

                if dx < 0 || dx >= self.width as isize {
                    continue;
                }

                unsafe {
                    let row = self.ptr.add(dy as usize * self.stride);
                    *row.add(dx as usize) = rgb565;
                }
            }
        }
    }

    /// Fills the entire framebuffer with zeros.
    pub fn zero(&self) {
        let pixels = self.stride * self.height;
        unsafe {
            std::ptr::write_bytes(self.ptr, 0, pixels);
        }
    }
}
