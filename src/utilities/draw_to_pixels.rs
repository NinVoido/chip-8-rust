pub fn draw_to_pixels(screen: [[bool; 64]; 32], pixels: &mut [u8]) {
    for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
        if screen[i / 64][i % 64] {
            pixel.copy_from_slice(&[255, 255, 255, 255])
        } else {
            pixel.copy_from_slice(&[0, 0, 0, 0])
        }
    }
}
