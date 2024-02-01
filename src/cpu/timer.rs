use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Clock {
    C4096,
    C262144,
    C65535,
    C16384
}

impl Clock {
    fn to_cycles(&self) -> u16 {
        match self {
            Clock::C4096 => 1024,
            Clock::C262144 => 16,
            Clock::C65535 => 64,
            Clock::C16384 => 256
        }
    }
}


pub struct Timer {
    div_counter: u8,
    timer_counter: u16,

    div: u8,
    tima: u8,
    tma: u8,
    tac: u8
}

impl Timer {

    pub fn new() -> Timer {
        Timer { div_counter: 0, timer_counter: 0, div: 0, tima: 0, tma: 0, tac:  0b100 }
    }

    fn is_enabled(&self) -> bool {
        (self.tac >> 2) != 0
    }

    fn get_frequency(&self) -> Clock {
        FromPrimitive::from_u8(self.tac & 0b11).unwrap()
    }

    fn set_frequency(&mut self, frequency: Clock) {
        self.tac = (self.tac & !(0b11)) | (frequency as u8);
    }

    pub fn update(&mut self, cycles: u8) -> bool {
        let (result, overflow) = self.div_counter.overflowing_add(4 * cycles);
        self.div_counter = result;
        if overflow {
            self.div_counter = 0;
            self.div = self.div.wrapping_add(1);
        }

        let interrupt = if self.is_enabled() {
            self.timer_counter = self.timer_counter.wrapping_add(4 * cycles as u16);

            let limit_cycles = self.get_frequency().to_cycles();
            if self.timer_counter > limit_cycles {
                self.timer_counter -= limit_cycles;
                let (result, overflow) = self.tima.overflowing_add(1);

                if overflow {
                    self.tima = self.tma;
                } else {
                    self.tima = result;
                }

                overflow
            } else {
                false
            }
        } else {
            return false;
        };
        interrupt
    }
}
