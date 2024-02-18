mod cpu;

use std::{fs, env};

use cpu::CPU;

fn read_rom(path: &str) -> Vec<u8> {
    fs::read(path).expect("File not found")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cpu = CPU::new();
    cpu.load_rom(read_rom(&args[1]));
    cpu.run();
}
