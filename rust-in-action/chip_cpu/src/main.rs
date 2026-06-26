use chip_cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 55;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 65);

    println!("55 + 10 = {}", cpu.registers[0]);
}
