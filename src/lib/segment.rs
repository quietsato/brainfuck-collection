use crate::lib::machine::Machine;

pub trait Exec {
    fn execute(&self, machine: &mut Machine);
}

pub enum FuncType {
    Inc,
    Dec,
    Next,
    Prev,
    Get,
    Put,
}

pub struct Func {
    func_type: FuncType,
}

impl Func {
    pub fn new(func_type: FuncType) -> Func {
        Func {
            func_type: func_type,
        }
    }
}

impl Exec for Func {
    fn execute(&self, machine: &mut Machine) {
        match self.func_type {
            FuncType::Inc => machine.inc(),
            FuncType::Dec => machine.dec(),
            FuncType::Next => machine.next(),
            FuncType::Prev => machine.prev(),
            FuncType::Get => {
                let stdin = std::io::stdin();
                let input: Option<u8> = stdin
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);

                match input {
                    Some(c) => machine.set_value(c),
                    None => {}
                };
            }
            FuncType::Put => {
                let c = machine.get_value();
                print!("{}", c as char);
            }
        }
    }
}

pub struct Block {
    inner: Vec<Func>
}

impl Block {
    pub fn new(inner: Vec<Func>) -> Block {
        Block { inner: inner }
    }
}

impl Exec for Block {
    fn execute(&self, machine: &mut Machine) {
        for e in self.inner {
            e.execute(&mut machine);
        }
    }
}

pub struct Loop {
    block: Block,
}

impl Loop {
    fn new(inner: Vec<Func>) -> Loop {
        Loop {
            block: Block::new(inner),
        }
    }
}

impl Exec for Loop {
    fn execute(&self, machine: &mut Machine) {
        if machine.get_value() == 0 {
            return;
        }
        loop {
            self.block.execute(&mut machine);
            if machine.get_value() != 0 {
                break;
            }
        }
    }
}

#[test]
#[should_panic]
fn sample_test() {
    println!("Hello World");
}
