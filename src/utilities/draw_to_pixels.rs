impl super::cpu::Cpu {
    pub fn draw_to_pixels(&mut self, pixels: &mut [u8]) {
        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            if self.screen[i / 64][i%64] {
                pixel.copy_from_slice(&[255, 255, 255, 255])
            } else {
                pixel.copy_from_slice(&[0, 0, 0, 0])
            }
        }
        self.redraw_needed = false
    }
}
