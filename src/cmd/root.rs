use std::fmt::Display;
use std::{env, process};

use super::{pack, unpack};

pub fn execute() {
    let mut args = env::args().skip(1);

    let Some(cmd) = args.next() else {
        print_help();
        return;
    };

    let result = match cmd.as_str() {
        "-h" | "--help" => {
            print_help();
            Ok(())
        }
        "pack" => pack::run(args.collect()),
        "unpack" => unpack::run(args.collect()),
        other => Err(format!("Unknown command: {other}")),
    };

    if let Err(err) = result {
        handle_err(&err);
    }
}

fn handle_err(err: impl Display) -> ! {
    eprintln!("{err}");
    process::exit(1);
}

fn print_help() {
    eprintln!(
        "\
Simple archiver

USAGE:
  packrs <COMMAND> [ARGS]

COMMANDS:
  pack      Pack file(s) into archive
  unpack    Unpack archive

GLOBAL OPTIONS:
  -h, --help  Print help

Try:
  packrs pack --help
  packrs unpack --help
"
    );
}
