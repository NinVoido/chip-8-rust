use winit::event::VirtualKeyCode::*;
impl super::cpu::Cpu {
    ///Update Chip's keypad if any of keypad keys were pressed
    pub fn update_keypad(&mut self, input: &winit_input_helper::WinitInputHelper) {
        if input.key_held(Key1) {
            self.keypad[1] = true
        } else {
            self.keypad[1] = false
        }

        if input.key_held(Key2) {
            self.keypad[2] = true
        } else {
            self.keypad[2] = false
        }

        if input.key_held(Key3) {
            self.keypad[3] = true
        } else {
            self.keypad[3] = false
        }

        if input.key_held(Key4) {
            self.keypad[0xc] = true
        } else {
            self.keypad[0xC] = false
        }

        if input.key_held(Q) {
            self.keypad[4] = true
        } else {
            self.keypad[4] = false
        }

        if input.key_held(W) {
            self.keypad[5] = true
        } else {
            self.keypad[5] = false
        }

        if input.key_held(E) {
            self.keypad[6] = true
        } else {
            self.keypad[6] = false
        }

        if input.key_held(R) {
            self.keypad[0xd] = true
        } else {
            self.keypad[0xd] = false
        }

        if input.key_held(A) {
            self.keypad[7] = true
        } else {
            self.keypad[7] = false
        }

        if input.key_held(S) {
            self.keypad[8] = true
        } else {
            self.keypad[8] = false
        }

        if input.key_held(D) {
            self.keypad[9] = true
        } else {
            self.keypad[9] = false
        }

        if input.key_held(F) {
            self.keypad[0xe] = true
        } else {
            self.keypad[0xe] = false
        }

        if input.key_held(Z) {
            self.keypad[0xa] = true
        } else {
            self.keypad[0xa] = false
        }

        if input.key_held(X) {
            self.keypad[0] = true
        } else {
            self.keypad[0] = false
        }

        if input.key_held(C) {
            self.keypad[0xb] = true
        } else {
            self.keypad[0xb] = false
        }

        if input.key_held(V) {
            self.keypad[0xf] = true
        } else {
            self.keypad[0xf] = false
        }
    }
}
