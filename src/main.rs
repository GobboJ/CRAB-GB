mod cpu;

use cpu::CPU;

fn main() {

    let mut cpu = CPU::new();
    cpu.run();
}
