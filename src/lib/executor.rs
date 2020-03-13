use crate::lib::machine::Machine;

pub struct Executor {
    pairs: Vec<usize>,
    src: Vec<char>,
    machine: Machine,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            pairs: Vec::new(),
            src: Vec::new(),
            machine: Machine::new(),
        }
    }

    pub fn append_src(&mut self, src: String) {
        self.src
            .append(&mut src.chars().filter(|c| is_func(*c)).collect());
    }

    pub fn execute(&mut self) -> Result<u8, String> {
        let mut i = 0;
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
                        let src = self.src.iter().collect::<String>();
                        match find_close_bracket(i, src.as_str()){
                            Some(close_pos) => i = close_pos,
                            None => return Err(format!("Mismatched brackets at {}", i)),
                        }
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
        }

        if self.pairs.is_empty() {
            Ok(0)
        } else {
            Err(format!("Mismatched brackets at {:?}", self.pairs))
        }
    }
}

fn is_func(c: char) -> bool {
    String::from("+-><,.[]").contains(c)
}

fn find_close_bracket(open_pos: usize, src: &str) -> Option<usize> {
    let mut nest = 0;

    for (i, c) in src.chars().enumerate().skip(open_pos){
        match c {
            '[' => {
                nest += 1;
            }
            ']' => {
                nest -= 1;
                if nest == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }

    None
}

#[test]
fn test_find_close_bracket() {
    let valid_src = "...[...]...";
    assert_eq!(find_close_bracket(3, &valid_src), Some(7));

    let valid_src = "...[.[].]...";
    assert_eq!(find_close_bracket(3, &valid_src), Some(8));

    let invalid_src = "...[...";
    assert_eq!(find_close_bracket(3, &invalid_src), None);
}

