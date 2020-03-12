use crate::lib::machine::*;

pub struct Executor {
    pairs: Vec<usize>,
    src: Vec<char>,
    pos: usize,
    skip_nest: usize,
    machine: Machine,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            pairs: Vec::new(),
            src: Vec::new(),
            pos: 0,
            skip_nest: 0,
            machine: Machine::new(),
        }
    }

    pub fn append_src(&mut self, src: &mut String) {
        self.src
            .append(&mut src.chars().filter(|c| is_func(c)).collect());
    }

    fn skip_while_pair(&mut self) -> Option<usize> {
        let mut i = self.pos;
        while i < self.src.len() {
            match self.src[i] {
                '[' => {
                    self.skip_nest += 1;
                }
                ']' => {
                    self.skip_nest -= 1;
                    if self.skip_nest == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }

            i += 1;
        }

        return None;
    }

    pub fn execute(&mut self) -> Result<u8, String> {
        if self.skip_nest > 0 {
            match self.skip_while_pair() {
                Some(pos) => self.pos = pos,
                None => {}
            }
        }
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
                    if self.machine.get_value() == 0 {
                        // skip loop
                        self.skip_while_pair();
                    } else {
                        // push start index to list
                        self.pairs.push(i);
                    }
                }
                ']' => {
                    // pop start index from list
                    let start = self.pairs.pop();
                    match start {
                        Some(s) => {
                            if self.machine.get_value() != 0 {
                                // if value != 0, continue loop
                                i = s;
                                self.pairs.push(s);
                            }
                        }
                        None => {
                            return Err(format!("Mismatched brackets at {}", i));
                        }
                    }
                }
                _ => {}
            }

            i += 1;
            self.pos = i;
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

#[test]
fn execute_test() {
    let mut e = Executor::new();

    let mut src = String::from("+++++.");
    e.append_src(&mut src);
    e.execute();
}
