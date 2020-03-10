pub struct Machine {
    mem: Vec<u8>,
    pos: usize,
}

impl Machine {
    pub fn new() -> Machine {
        let size = 30000;
        Machine {
            mem: vec![0; size],
            pos: 0,
        }
    }

    pub fn get_value(&self) -> u8 {
        self.mem[self.pos]
    }

    pub fn set_value(&mut self, v: u8) {
        self.mem[self.pos] = v;
    }

    pub fn inc(&mut self) {
        if self.mem[self.pos] == std::u8::MAX{
            self.mem[self.pos] = std::u8::MIN;
        } else {
            self.mem[self.pos] += 1;
        }
    }

    pub fn dec(&mut self) {
        if self.mem[self.pos] == std::u8::MIN {
            self.mem[self.pos] = std::u8::MAX;
        } else {
            self.mem[self.pos] -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.pos == self.mem.len() - 1 {
            self.pos = 0;
        } else {
            self.pos += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.pos == 0 {
            self.pos = self.mem.len() - 1;
        } else {
            self.pos += 1;
        }
    }
}
