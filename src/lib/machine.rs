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

    pub fn get(&mut self) {
        use std::io::*;
        let input: Option<u8> = stdin()
            .bytes()
            .next()
            .and_then(|res| res.ok())
            .map(|byte| byte as u8);

        match input {
            Some(c) => self.mem[self.pos] = c,
            None => {}
        };
    }

    pub fn put(&mut self) {
        let c = self.mem[self.pos];
        print!("{}", c as char);
    }

    pub fn inc(&mut self) {
        if self.mem[self.pos] == std::u8::MAX {
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
            self.pos -= 1;
        }
    }
}
