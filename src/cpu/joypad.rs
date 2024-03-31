pub enum Button {
    A,
    B,
    SEL,
    STA,
    R,
    L,
    U,
    D
}

pub struct Joypad {
    register: u8
}

impl Joypad {

    pub fn new() -> Self {
        Joypad { register: 0xFF }
    }

    pub fn read_register(&self) -> u8 {
        // println!("{:#010b} *", self.register);
        if (self.register >> 4) & 0b11 == 0b11 {
            self.register | 0xF 
        }
        else {
            self.register
        }
    }

    pub fn write_register(&mut self, value: u8) {
        println!("{:#010b} -> {:#010b}", self.register, value);
        self.register = value | (self.register & 0b1111);
        println!("-> {:#010b}", self.register);
    }

    pub fn set_button(&mut self, button: Button) -> bool {
        // println!("{:#010b} *", self.register);
        let old_value = self.register;
        match button {
            Button::A | Button::R => self.register &= !(1),
            Button::B | Button::L => self.register &= !(1 << 1),
            Button::SEL | Button::U => self.register &= !(1 << 2),
            Button::STA | Button::D => self.register &= !(1 << 3),
        }
        // println!("{:#010b} ->", self.register);

        let request_joypad = if old_value != self.register {
            let is_dpad = match button {
                Button::R | Button::L | Button::U | Button::D => true,
                _ => false
            };

            (is_dpad && (self.register >> 4 & 1) == 0) || (!is_dpad && (self.register >> 5 & 1) == 0)
        } else {
            false
        };

        request_joypad
    }

    pub fn unset_button(&mut self, button: Button) {
        match button {
            Button::A | Button::R => self.register |= 1,
            Button::B | Button::L => self.register |= 1 << 1,
            Button::SEL | Button::U => self.register |= 1 << 2,
            Button::STA | Button::D => self.register |= 1 << 3,
        }
    }
}
