use chip_cpu::cpu::CPU;

fn main() {
    // let mut cpu = CPU::new();

    // cpu.current_operation = 0x8014;
    // cpu.registers[0] = 55;
    // cpu.registers[1] = 10;

    // cpu.run();

    // assert_eq!(cpu.registers[0], 65);

    // println!("55 + 10 = {}", cpu.registers[0]);

    test_is_bit_on();
}


fn test_is_bit_on() {
    let num_bit = 0b0011_0000_1111_0110;
    let mask = 0b0010_0000_0000_0010;

    println!("{} | {}", num_bit, mask);
    println!("{:0b} | {:0b}", num_bit, mask);

    let one_at_bit = num_bit & mask;

    println!("{} | {:0b}", one_at_bit, one_at_bit);

    if one_at_bit != 0{
        println!("not equal to 0");
    }else {
        println!("equal to 0");
    }
}
