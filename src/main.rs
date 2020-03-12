mod lib;

use lib::executor::*;

fn main() {
    // prints "Hello World."
    // http://unidentifiedexe.hatenablog.com/entry/2019/10/10/232431
    let mut src = String::from(
        r#"
        ++++++++++[->++++++++++>+++>++++++++++++>+++++>+++++++<<<<<]
        >>>>>++.<<<<+.+++++++..+++.>++.>-.<<.+++.------.--------.>>>----.
        "#,
    );

    let mut e = Executor::new();

    e.append_src(&mut src);

    e.execute();
    e.notify_end_of_src();
}
