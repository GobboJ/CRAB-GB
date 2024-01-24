use std::fs;


struct Bootrom {
    code: [u8; 0x100],
    enabled: bool
}

impl Bootrom {

    pub fn new() -> Bootrom {
        let mut bootrom = Bootrom {
            code: [0; 0x100],
            enabled: true
        };

        bootrom.load_bootrom();
        bootrom
    }

    fn load_bootrom(&mut self) {
        let _data = fs::read("src/cpu/bootix_dmg.bin").expect("Bootrom image not found!");
        self.code.copy_from_slice(&_data);
    }

    fn read(&self, address: u16) -> u8 {
        self.code[address as usize]
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_disable(&mut self) {
        self.enabled = false;
    }
}

pub struct Memory {

    bootrom: Bootrom,
    rom_bank_0: [u8; 0x4000],
    rom_bank_n: [u8; 0x4000],
    video_ram: [u8; 0x2000],
    external_ram: [u8; 0x2000],
    work_ram: [u8; 0x2000],

    io_registers: [u8; 0x80],
    high_ram: [u8; 0x7F]
}

impl Memory {

    pub fn new() -> Memory {
        let mut memory = Memory {
            bootrom: Bootrom::new(),
            rom_bank_0: [0; 0x4000],
            rom_bank_n: [0; 0x4000],
            video_ram: [0; 0x2000],
            external_ram: [0; 0x2000],
            work_ram: [0; 0x2000],
            io_registers: [0; 0x80],
            high_ram: [0; 0x7F]
        };

        memory.load_rom();

        memory
    }


    pub fn load_rom(&mut self) {
        let data = fs::read("roms/01-special.gb").expect("Rom image not found!");
        let (bank_0, bank_1) = data.split_at(0x4000);
        self.rom_bank_0.copy_from_slice(bank_0);
        self.rom_bank_n.copy_from_slice(bank_1);
    }

    fn read_bootrom(&self, address: u16) -> u8 {
        self.bootrom.read(address)
    }

    fn read_rom_bank_0(&self, address: u16) -> u8 {
        self.rom_bank_0[address as usize]
    }

    fn read_rom_bank_n(&self, address: u16) -> u8 {
        self.rom_bank_n[address as usize]
    }

    fn read_video_ram(&self, address: u16) -> u8 {
        self.video_ram[address as usize]
    }

    fn read_external_ram(&self, address: u16) -> u8 {
        self.external_ram[address as usize]
    }
    
    fn read_work_ram(&self, address: u16) -> u8 {
        self.work_ram[address as usize]
    }

    fn read_io_registers(&self, address: u16) -> u8 {
        self.io_registers[address as usize]
    }

    fn read_high_ram(&self, address: u16) -> u8 {
        self.high_ram[address as usize]
    }

    fn write_external_ram(&mut self, address: u16, data: u8) {
        self.external_ram[address as usize] = data;
    }

    fn write_work_ram(&mut self, address: u16, data: u8) {
        self.external_ram[address as usize] = data;
    }

    fn write_video_ram(&mut self, address: u16, data: u8) {
        self.video_ram[address as usize] = data;
    }

    fn write_io_registers(&mut self, address: u16, data: u8) {
        self.io_registers[address as usize] = data;
    }

    fn write_high_ram(&mut self, address: u16, data: u8) {
        self.high_ram[address as usize] = data;
    }
    
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF => {
                if self.bootrom.is_enabled() {
                    self.bootrom.read(address)
                } else {
                    self.read_rom_bank_0(address)
                }
            }
            0x0000..=0x3FFF => self.read_rom_bank_0(address),
            0x4000..=0x7FFF => self.read_rom_bank_n(address - 0x4000),
            0x8000..=0x9FFF => self.read_video_ram(address - 0x8000),
            0xA000..=0xBFFF => self.read_external_ram(address - 0xA000),
            0xC000..=0xDFFF => self.read_work_ram(address - 0xC000),
            0xFF00..=0xFF7F => self.read_io_registers(address - 0xFF00),
            0xFF80..=0xFFFE => self.read_high_ram(address - 0xFF80),
            x => panic!("Accessed reading unimplemented area: {:x}", x)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.write_video_ram(address - 0x8000, data),
            0xA000..=0xBFFF => self.write_external_ram(address - 0xA000, data),
            0xC000..=0xDFFF => self.write_work_ram(address - 0xC000, data),
            0xFF00..=0xFF7F => {
                self.write_io_registers(address - 0xFF00, data);
                if let 0xFF50 = address {
                    println!("Disabled bootrom!");
                    self.bootrom.set_disable();
                }
            },
            0xFF80..=0xFFFE => self.write_high_ram(address - 0xFF80, data),
            x => panic!("Accessed writing unimplemented area: {:x}", x)
        }
    }
}
