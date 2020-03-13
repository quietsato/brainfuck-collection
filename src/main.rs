extern crate clap;
use clap::{App, Arg};

mod lib;
use lib::executor::Executor;

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
    if let Some(src) = matches.value_of("SOURCE") {
        let mut e = Executor::new();

        e.append_src(src.to_string());
        e.execute().unwrap();
    }

    // exec source file


    // exec source from stdin
}
