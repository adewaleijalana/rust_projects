use chip_cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    // cpu.registers[2] = 10;
    // cpu.registers[3] = 10;

    // cpu.memory[0] = 0x80;
    // cpu.memory[1] = 0x14;
    // cpu.memory[2] = 0x80;
    // cpu.memory[3] = 0x24;
    // cpu.memory[4] = 0x80;
    // cpu.memory[5] = 0x34;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    // assert_eq!(cpu.registers[0], 35);

    // println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);

    // test_is_bit_on();
}
