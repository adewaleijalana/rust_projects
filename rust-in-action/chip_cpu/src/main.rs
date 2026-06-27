use chip_cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

    // test_is_bit_on();
}

fn test_is_bit_on() {
    let arg1 = 23_i64;
    let result = arg1.overflowing_add(23);

    let num_bit = 0b0011_0000_1111_0110;
    let mask = 0b0010_0000_0000_0010;

    println!("{} | {}", num_bit, mask);
    println!("{:0b} | {:0b}", num_bit, mask);

    let one_at_bit = num_bit & mask;

    println!("{} | {:0b}", one_at_bit, one_at_bit);

    if one_at_bit != 0 {
        println!("not equal to 0");
    } else {
        println!("equal to 0");
    }
}
