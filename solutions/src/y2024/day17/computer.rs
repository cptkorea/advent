pub struct Computer {
    pub instructions: Vec<u8>,
    pub registers: Registers,
    pub pointer: usize,
    pub outputs: Vec<u64>,
}

pub struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Computer {
    pub fn new(instructions: Vec<u8>, a: u64, b: u64, c: u64) -> Self {
        Self {
            instructions,
            registers: Registers { a, b, c },
            pointer: 0,
            outputs: Vec::new(),
        }
    }

    pub fn restart(&mut self, a: u64, b: u64, c: u64) {
        self.registers.a = a;
        self.registers.b = b;
        self.registers.c = c;
        self.pointer = 0;
        self.outputs = Vec::new();
    }

    pub fn run(&mut self) {
        while self.pointer < self.instructions.len() {
            let i = self.pointer;
            let (instruction, operand) = (self.instructions[i], self.instructions[i + 1]);

            let combo = match operand {
                x @ 0..=3 => x as u64,
                4 => self.registers.a,
                5 => self.registers.b,
                6 => self.registers.c,
                _ => unimplemented!("unimplemented value"),
            };

            match instruction {
                0 => self.adv(combo),
                1 => self.bxl(operand as u64),
                2 => self.bst(combo),
                3 => self.jnz(operand as u64),
                4 => self.bxc(combo),
                5 => self.out(combo),
                6 => self.bdv(combo),
                7 => self.cdv(combo),
                _ => unimplemented!("unknown instruction"),
            }

            if self.pointer == i {
                self.pointer += 2;
            }
        }
    }

    pub fn outputs(&self) -> &Vec<u64> {
        &self.outputs
    }

    pub fn concat_output(&self) -> String {
        let mut total = String::new();
        for out in &self.outputs {
            total.push_str(&out.to_string());
        }
        total
    }

    fn adv(&mut self, combo: u64) {
        let a = self.registers.a;
        let denom = 2_u64.pow(combo as u32);
        self.registers.a = a / denom
    }

    fn bxl(&mut self, op: u64) {
        self.registers.b = self.registers.b ^ op;
    }

    fn bst(&mut self, combo: u64) {
        self.registers.b = combo % 8;
    }

    fn jnz(&mut self, op: u64) {
        if self.registers.a != 0 {
            self.pointer = op as usize;
        }
    }

    fn bxc(&mut self, _: u64) {
        self.registers.b = self.registers.b ^ self.registers.c;
    }

    fn out(&mut self, combo: u64) {
        self.outputs.push(combo % 8);
    }

    fn bdv(&mut self, combo: u64) {
        let a = self.registers.a;
        let denom = 2_u64.pow(combo as u32);
        self.registers.b = a / denom
    }

    fn cdv(&mut self, combo: u64) {
        let a = self.registers.a;
        let denom = 2_u64.pow(combo as u32);
        self.registers.c = a / denom
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let instructions = vec![0, 1, 5, 4, 3, 0];
        let mut computer = Computer::new(instructions, 729, 0, 0);
        computer.run();
        assert_eq!("4635635210", computer.concat_output());
    }
}
