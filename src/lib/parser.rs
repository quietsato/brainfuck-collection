use core::iter::Enumerate;
use crate::lib::segment::*;

pub fn parse(src: String) -> Option<Block> {
    // find bracket pairs
    let mut pairs: Vec<Vec<(usize, usize)>> = Vec::new();

    // wrap funcs
    let mut funcs: Vec<Func> = Vec::new();
    for (i, c) in src.chars().enumerate() {
        match c {
            '+' => funcs.push(Func::new(FuncType::Inc)),
            '-' => funcs.push(Func::new(FuncType::Dec)),
            '>' => funcs.push(Func::new(FuncType::Next)),
            '<' => funcs.push(Func::new(FuncType::Prev)),
            ',' => funcs.push(Func::new(FuncType::Get)),
            '.' => funcs.push(Func::new(FuncType::Put)),
        }
    }
    unimplemented!();
}

fn find_pair(
    pairs: &mut Vec<Vec<(usize, usize)>>,
    src_iter: Enumerate<char>,
    root: bool,
    start_index: usize,
    nest: usize,
) {
    for (i, c) in src_iter {

    }
}
