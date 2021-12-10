// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

mod agent;
mod cmd_parser;
mod destination;

use agent::Agent;
use cmd_parser::CmdParser;

fn main() {
    let cmd_parser = CmdParser::new();
    let dest = cmd_parser.dest().unwrap();
    let input = cmd_parser.foreign_source_input().unwrap();

    let mut agent = Agent::new(dest).unwrap();
    for j in input {
        let j = j.unwrap();
        agent.write(j.to_string()).unwrap()
    }
}
