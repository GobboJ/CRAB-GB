use std::fs;

use super::timer::Timer;
use super::interrupt::Interrupt;
use super::gpu::GPU;

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
    external_ram: [u8; 0x2000],
    work_ram: [u8; 0x2000],

    // io_registers: [u8; 0x80],
    high_ram: [u8; 0x7F],

    timer: Timer,
    interrupt: Interrupt,
    gpu: GPU
}

impl Memory {

    pub fn new() -> Memory {
        let mut memory = Memory {
            bootrom: Bootrom::new(),
            rom_bank_0: [0; 0x4000],
            rom_bank_n: [0; 0x4000],
            external_ram: [0; 0x2000],
            work_ram: [0; 0x2000],
            // io_registers: [0; 0x80],
            high_ram: [0; 0x7F],

            timer: Timer::new(),
            interrupt: Interrupt::new(),
            gpu: GPU::new()
        };
        memory
    }


    pub fn load_rom(&mut self, data: Vec<u8>) {
        let (bank_0, bank_1) = data.split_at(0x4000);
        self.rom_bank_0.copy_from_slice(bank_0);
        self.rom_bank_n.copy_from_slice(bank_1);
    }

    fn read_rom_bank_0(&self, address: u16) -> u8 {
        self.rom_bank_0[address as usize]
    }

    fn read_rom_bank_n(&self, address: u16) -> u8 {
        self.rom_bank_n[address as usize]
    }

    fn read_video_ram(&self, address: u16) -> u8 {
        self.gpu.read_vram(address)
    }

    fn read_external_ram(&self, address: u16) -> u8 {
        self.external_ram[address as usize]
    }
    
    fn read_work_ram(&self, address: u16) -> u8 {
        self.work_ram[address as usize]
    }

    // fn read_io_registers(&self, address: u16) -> u8 {
    //     self.io_registers[address as usize]
    // }

    fn read_high_ram(&self, address: u16) -> u8 {
        self.high_ram[address as usize]
    }

    fn write_external_ram(&mut self, address: u16, data: u8) {
        self.external_ram[address as usize] = data;
    }

    fn write_work_ram(&mut self, address: u16, data: u8) {
        self.work_ram[address as usize] = data;
    }

    fn write_video_ram(&mut self, address: u16, data: u8) {
        self.gpu.write_vram(address, data);
    }

    // fn write_io_registers(&mut self, address: u16, data: u8) {
    //     self.io_registers[address as usize] = data;
    // }

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
            0xE000..=0xFDFF => self.read_work_ram(address - 0xE000),
            0xFF00..=0xFF7F => self.handle_read_io_register(address),
            0xFF80..=0xFFFE => self.read_high_ram(address - 0xFF80),
            0xFFFF => self.interrupt.read_interrupt_enable(),
            x => panic!("Accessed reading unimplemented area: {:x}", x)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.write_video_ram(address - 0x8000, data),
            0xA000..=0xBFFF => self.write_external_ram(address - 0xA000, data),
            0xC000..=0xDFFF => self.write_work_ram(address - 0xC000, data),
            0xFF00..=0xFF7F => self.handle_write_io_register(address, data),
            0xFF80..=0xFFFE => self.write_high_ram(address - 0xFF80, data),
            0xFFFF => self.interrupt.write_interrupt_enable(data),
            x => panic!("Accessed writing unimplemented area: {:x}", x)
        }
    }


    fn handle_read_io_register(&self, address: u16) -> u8 {
        let res = match address {
            0xFF0F => self.interrupt.read_interrupt_flag(),
            0xFF44 => self.gpu.read_ly(),
            x => panic!("Reading unknown IO Register {:x}", x)
        };
        // println!("[IO REA] {:#06x} = {}", address, res);
        res
    }

    fn handle_write_io_register(&mut self, address: u16, data: u8) {
        // println!("[IO WRI] {:#06x} = {:#04x}", address, data);
        match address {
            0xFF01 => print!("{}", data as char),
            0xFF02 => {},
            0xFF05 => self.timer.write_tima(data),
            0xFF06 => self.timer.write_tma(data),
            0xFF07 => self.timer.write_tac(data),
            0xFF0F => self.interrupt.write_interrupt_flag(data),
            0xFF10..=0xFF26 => {}, // Audio
            0xFF40 => self.gpu.write_lcd_control(data),
            0xFF41 => self.gpu.write_lcd_status(data),
            0xFF42 => self.gpu.write_scy(data),
            0xFF43 => self.gpu.write_scx(data),
            0xFF47 => self.gpu.write_bgp(data),
            0xFF50 => {
                println!("Disabled bootrom!");
                self.bootrom.set_disable();
            },
            x => panic!("Writing unknown IO Register {:x}", x)
        }
    }

    pub fn update_timer(&mut self, cycles: u8) {
        let interrupt = self.timer.update(cycles);
        if interrupt {
            self.get_interrupts().write_bit_interrupt_flag(&super::interrupt::InterruptHandler::Timer, true);
        }
    }

    pub fn update_gpu(&mut self, cycles: u8) {
        let (vblank, lcd) = self.gpu.update(cycles);
        if vblank {
            // println!("SET VBLANK INTERRUPT FLAG");
            self.get_interrupts().write_bit_interrupt_flag(&super::interrupt::InterruptHandler::VBlank, true);
        }
        if lcd {
            self.get_interrupts().write_bit_interrupt_flag(&super::interrupt::InterruptHandler::LCD, true);
        }
    }

    pub fn get_interrupts(&mut self) -> &mut Interrupt {
        &mut self.interrupt
    }
}
