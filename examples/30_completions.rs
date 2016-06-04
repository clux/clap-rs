#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

// cargo build --example 30_completions --features completion
// ./target/debug/examples/30_completions > examples/30_completions.sh
fn main() {
    let app = App::new("foo")
        .version("0.0.1")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Use verbose output"))
        .subcommand(SubCommand::with_name("bar")
            .arg(Arg::with_name("barable")
                .required(true)
                .multiple(true))
            .arg(Arg::with_name("save")
                .short("S")
                .long("save"))
            .arg(Arg::with_name("discard")
                .short("D")
                .long("discard")))
        .subcommand(SubCommand::with_name("baz")
            .arg(Arg::with_name("bazable"))
            .arg(Arg::with_name("config")
                .long("config")
                .short("c")
                .takes_value(true))
            .arg(Arg::with_name("strict")
                .long("strict")
                .short("s"))
            .arg(Arg::with_name("petty")
                .long("petty")
                .short("p")))
        .subcommand(SubCommand::with_name("qux")
            .subcommand(SubCommand::with_name("corge")
                .arg(Arg::with_name("privileged")
                    .short("p")
                    .long("privileged")))
            .subcommand(SubCommand::with_name("grault")
                .arg(Arg::with_name("graults")
                    .multiple(true)
                    .required(true))));
    let res = app.completions();
    println!("{}", res);
}
