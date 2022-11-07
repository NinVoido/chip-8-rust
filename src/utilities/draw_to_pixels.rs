impl super::cpu::Cpu {
    pub fn draw_to_pixels(&mut self, pixels: &mut [u8]) {
        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            if self.highres {
                if self.screen[i / 128][i % 128] {
                    pixel.copy_from_slice(&[255, 255, 255, 255])
                } else {
                    pixel.copy_from_slice(&[0, 0, 0, 0])
                }
            } else {
                let mut addr = i;
                if addr % 2 == 1 {
                    addr -= 1;
                }
                addr /= 2;
                if self.screen[addr / 128][addr % 64] {
                    pixel.copy_from_slice(&[255, 255, 255, 255])
                } else {
                    pixel.copy_from_slice(&[0, 0, 0, 0])
                }
            }
        }
        self.redraw_needed = false
    }
}
