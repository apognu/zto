#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate clap;
extern crate zto;

use clap::{App, SubCommand};

use zto::command::{common, controller, network, peer};
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    if let None = args.subcommand {
        eprintln!("ERROR: at least one command must be given");
        process::exit(1);
    }

    match args.subcommand.unwrap() {
        box SubCommand { name, matches: args @ _, } => {
            let command: Box<common::Command> = match name.as_ref() {
                "network" => box network::Network {},
                "controller" => box controller::Controller {},
                "peer" => box peer::Peer {},
                _ => {
                    eprintln!("ERROR: unknown command `{}`", name);
                    process::exit(1);
                }
            };

            command.handle(args);
        }
    }
}
