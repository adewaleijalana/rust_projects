pub struct CPU {
    resgister: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            resgister: [0; 16],
            position_in_memory: 0,
            memory: [0; 0x1000],
        }
    }

    pub fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        //loop {
        let opcode = self.read_opcode();
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
          (0, 0, 0, 0) => { return; },
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }

        //}
    }

    fn add_xy(&mut self, x: u8, y: u8) {
       let arg1 = self.resgister[x as usize];
       let arg2 = self.resgister[y as usize];

       let (val, overflow) = arg1.overflowing_add(arg2);
       self.resgister[x as usize] = val;

       if overflow {
           self.resgister[0xf] = 1;
       }else {
           self.resgister[0xf] = 0;
       }
    }

    // fn add_xy(&mut self, x: u8, y: u8) {
    //     self.registers[x as usize] += self.registers[y as usize];
    // }
}
