// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

mod cmd_parser;

use cmd_parser::CmdParser;

fn main() {
    let cmd_parser = CmdParser::new();
    let input = cmd_parser.foreign_source_input().unwrap();

    for j in input {
        println!("{}", j.unwrap());
    }
}
