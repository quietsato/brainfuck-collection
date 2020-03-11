use crate::lib::machine::*;
pub struct Executor {
    pairs: Vec<usize>,
    src: Vec<char>,
    pos: usize,
    machine: Machine,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            pairs: Vec::new(),
            src: Vec::new(),
            pos: 0,
            machine: Machine::new(),
        }
    }

    pub fn append_src(&mut self, src: &mut String) {
        self.src
            .append(&mut src.chars().filter(|c| is_func(c)).collect());
    }

    pub fn execute(&mut self) -> Result<u8, String> {
        let mut i = self.pos;
        while i < self.src.len() {
            match self.src[i] {
                '+' => self.machine.inc(),
                '-' => self.machine.dec(),
                '>' => self.machine.next(),
                '<' => self.machine.prev(),
                ',' => self.machine.get(),
                '.' => self.machine.put(),
                '[' => {
                    // push start index to list
                }
                ']' => {
                    // pop start index from list
                    // set self.pos to start index
                }
                _ => {}
            }

            i += 1;
        }

        if self.pairs.is_empty() {
            Ok(0)
        } else {
            Ok(1)
        }
    }

    pub fn notify_end_of_src(self) -> Result<u8, String> {
        if self.pairs.is_empty() {
            Ok(0)
        } else {
            Err(format!("Mismatched brackets at {:?}", self.pairs))
        }
    }
}

fn is_func(c: &char) -> bool {
    match c {
        '+' | '-' | '>' | '<' | ',' | '.' | '[' | ']' => true,
        _ => false,
    }
}
