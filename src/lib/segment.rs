trait Exec {
    fn execute(&self, machine: Machine);
}

struct Func{
    name: char
}

impl Exec for Func {
    fn execute(&self, machine: &mut Machine){
    }
}

struct Block {
    inner: Vec<Block>
}

