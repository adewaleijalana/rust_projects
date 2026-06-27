use std::error::Error;

pub fn test_is_bit_on() {
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

fn combine_opcodes(opcode: &[u8]) -> Result<u16, Box<dyn Error>> {
    if opcode.len() % 2 != 0 {
        Err("Invalid data".into())
    } else {
        let arg1 = opcode[0];
        let arg2 = opcode[1];

        println!("arg1 as u8: {} | {:08b}", arg1, arg1);

        let arg1 = arg1 as u16;

        // println!("arg1 as u16: {} | {:016b}", arg1, arg1);

        // println!("arg2 as u8: {} | {:08b}", arg1, arg1);

        let arg2 = arg2 as u16;

        // println!("arg2 as u16: {} | {:016b}", arg2, arg2);

        let arg1_shift_8 = arg1 << 8;
        // println!(
        //     "arg1 after 8 bit shifts: {} | {:016b} | {:#06x}",
        //     arg1_shift_8, arg1_shift_8, arg1_shift_8
        // );

        let combined_val = arg1_shift_8 | arg2;
        // println!(
        //     "combined args: {} | {:016b} | {:#06x}",
        //     combined_val, combined_val, combined_val
        // );

        Ok(combined_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[should_panic(expected = "Invalid data")]
    fn should_panic_combine_opcodes_test() {
        let _ = combine_opcodes(&vec![12]).unwrap();
    }

    #[rstest]
    #[case(&vec![0x80, 0x14], 0x8014)]
    #[case(&vec![0x12, 0x13], 0x1213)]
    fn combine_opcodes_test(#[case] a: &[u8], #[case] expected: u16) {
        // let res = combine_opcodes(&vec![0x80, 0x14])?;
        // println!("result: {}", res);
        assert_eq!(combine_opcodes(a).unwrap(), expected);
    }
}
