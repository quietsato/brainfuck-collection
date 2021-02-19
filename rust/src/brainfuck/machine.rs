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

        if let Some(c) = input {
            self.mem[self.pos] = c
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

#[test]
fn machine_initialize_test() {
    let m = Machine::new();

    let size = 30000;
    // initial position
    assert_eq!(m.pos, 0);
    // initial size
    assert_eq!(m.mem.len(), size);

    // all values are 0
    assert_eq!(m.mem[0], 0);
    assert_eq!(m.mem[5000], 0);
    assert_eq!(m.mem[size - 1], 0);
}

#[test]
fn machine_function_inc() {
    let mut m = Machine::new();

    assert_eq!(m.mem[0], 0);

    m.inc();
    assert_eq!(m.mem[0], 1);

    use std::u8::MAX;
    m.mem[0] = MAX;
    m.inc();
    assert_eq!(m.mem[0], 0);
}

#[test]
fn machine_function_dec() {
    let mut m = Machine::new();

    m.mem[0] = 1;
    assert_eq!(m.mem[0], 1);

    m.dec();
    assert_eq!(m.mem[0], 0);

    use std::u8::MAX;
    m.dec();
    assert_eq!(m.mem[0], MAX);
}

#[test]
fn machine_function_next() {
    let mut m = Machine::new();

    assert_eq!(m.pos, 0);

    m.next();
    assert_eq!(m.pos, 1);

    m.pos = m.mem.len() - 1;
    m.next();
    assert_eq!(m.pos, 0);
}

#[test]
fn machine_function_prev() {
    let mut m = Machine::new();

    m.pos = 1;
    assert_eq!(m.pos, 1);

    m.prev();
    assert_eq!(m.pos, 0);

    m.prev();
    assert_eq!(m.pos, m.mem.len() - 1);
}

