mod lib;

use lib::executor::*;

fn main() {
    // prints brainfuck
    let mut src = String::from(
        r#"
++++++++[>++++++++<-]>++.
<++++++[>++++++++<-]>.
<----[>++++<-]>-.
++++++++.
+++++.
--------.
+++++++++++++++.
<----[>++++<-]>--.
++++++++.
    "#,
    );

    let mut e = Executor::new();

    e.append_src(&mut src);

    e.execute();
    e.notify_end_of_src();
}
