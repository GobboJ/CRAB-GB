use std::fs;

pub struct Memory {

    bootrom: [u8; 0x100],
    rom_bank_0: [u8; 0x4000],
    rom_bank_n: [u8; 0x4000],
    external_ram: [u8; 0x2000],
    work_ram: [u8; 0x2000],
    
}

impl Memory {

    pub fn new() -> Memory {
        let mut memory = Memory {
            bootrom: [0; 0x100],
            rom_bank_0: [0; 0x4000],
            rom_bank_n: [0; 0x4000],
            external_ram: [0; 0x2000],
            work_ram: [0; 0x2000],
        };
        memory.load_bootrom();

        memory
    }

    fn load_bootrom(&mut self) {
        let _data = fs::read("src/cpu/bootix_dmg.bin").expect("Bootrom image not found!");
        self.bootrom.copy_from_slice(&_data);
    }

    pub fn load_rom(&self) {
        todo!()
    }
}
