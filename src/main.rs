use clap::{App, Arg};
use brainfuck::execute;

fn main() {
    let matches = App::new("BrainFuck Interpreter")
        .version("1.0")
        .author("lnsf <lnsf03@gmail.com>")
        .about("A BrainFuck interpreter")
        .arg(
            Arg::with_name("Execute")
                .short("e")
                .long("exec")
                .value_name("SOURCE")
                .help("Execute passed source code")
                .takes_value(true),
        )
        .arg(Arg::with_name("FILE").help("Specify source file").index(1))
        .get_matches();

    // exec source passed as command line argument
    if let Some(src) = matches.value_of("Execute") {
        execute(src).unwrap();
    }
    // exec source file
    else if let Some(file) = matches.value_of("FILE") {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(file).unwrap();
        let mut src = String::new();
        file.read_to_string(&mut src).unwrap();

        execute(src.as_str()).unwrap();
    }
    // exec source from stdin
    else {
        use std::io::*;
        let mut src = String::new();
        stdin().read_to_string(&mut src).unwrap();


        execute(src.as_str()).unwrap();
    }
}

