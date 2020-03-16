use crate::brainfuck::machine::Machine;

pub fn execute(src: &str) -> Result<u8, String> {
    let mut machine = Machine::new();
    let src: Vec<char> = src.chars().collect();
    let mut pairs: Vec<usize> = Vec::new();

    let mut i = 0;
    while i < src.len() {
        match src[i] {
            '+' => machine.inc(),
            '-' => machine.dec(),
            '>' => machine.next(),
            '<' => machine.prev(),
            ',' => machine.get(),
            '.' => machine.put(),
            '[' => {
                if machine.get_value() == 0 {
                    // skip loop
                    let src = src.iter().collect::<String>();
                    match find_close_bracket(i, src.as_str()) {
                        Some(close_pos) => i = close_pos,
                        None => return Err(format!("Mismatched brackets at {}", i)),
                    }
                } else {
                    // push start index to list
                    pairs.push(i);
                }
            }
            ']' => {
                // pop start index from list
                let start = pairs.pop();
                match start {
                    Some(s) => {
                        if machine.get_value() != 0 {
                            // if value != 0, continue loop
                            i = s;
                            pairs.push(s);
                        }
                    }
                    None => {
                        return Err(format!("Mismatched brackets at char {}", i));
                    }
                }
            }
            _ => {}
        }

        i += 1;
    }

    if pairs.is_empty() {
        Ok(0)
    } else {
        Err(format!("Mismatched brackets at char {:?}", pairs))
    }
}

fn find_close_bracket(open_pos: usize, src: &str) -> Option<usize> {
    let mut nest = 0;

    for (i, c) in src.chars().enumerate().skip(open_pos) {
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
