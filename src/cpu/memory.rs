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

    fn read_bootrom(&self, address: u16) -> u8 {
        self.bootrom[address as usize]
    }

    fn read_rom_bank_0(&self, address: u16) -> u8 {
        self.rom_bank_0[address as usize]
    }

    fn read_rom_bank_n(&self, address: u16) -> u8 {
        self.rom_bank_n[address as usize]
    }

    fn read_external_ram(&self, address: u16) -> u8 {
        self.external_ram[address as usize]
    }
    
    fn read_work_ram(&self, address: u16) -> u8 {
        self.work_ram[address as usize]
    }

    fn write_external_ram(&mut self, address: u16, data: u8) {
        self.external_ram[address as usize] = data;
    }

    fn write_work_ram(&mut self, address: u16, data: u8) {
        self.external_ram[address as usize] = data;
    }
    
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.read_bootrom(address),
            0x4000..=0x7FFF => self.read_rom_bank_n(address - 0x4000),
            0xA000..=0xBFFF => self.read_external_ram(address - 0xA000),
            0xD000..=0xDFFF => self.read_work_ram(address - 0xD000),
            x => panic!("Accessed unimplemented area: {:x}", x)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0xA000..=0xBFFF => self.write_external_ram(address - 0xA000, data),
            0xD000..=0xDFFF => self.write_work_ram(address - 0xD000, data),
            x => panic!("Accessed unimplemented area: {:x}", x)
        }
    }
}
