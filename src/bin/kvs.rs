extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("kvs")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("V")
                .short("V")
                .multiple(true)
                .help("Get the version"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value for a key")
                .arg(
                    Arg::with_name("key")
                        .value_name("key")
                        .min_values(1)
                        .max_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value for a key")
                .arg(Arg::with_name("key").value_name("key"))
                .arg(Arg::with_name("val").value_name("val")),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the value for a key")
                .arg(
                    Arg::with_name("key")
                        .value_name("key")
                        .min_values(1)
                        .max_values(1),
                ),
        )
        .get_matches();

    if matches.is_present("V") {
        println!(env!("CARGO_PKG_VERSION"));
    }

    if matches.is_present("get") {
        eprintln!("unimplemented");
        process::exit(1);
    }

    if matches.is_present("set") {
        eprintln!("unimplemented");
        process::exit(1);
    }

    if matches.is_present("rm") {
        eprintln!("unimplemented");
        process::exit(1);
    }

    process::exit(1);
}
