// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

mod agent;
mod cmd_parser;
mod destination;
mod timed_stream;

use agent::Agent;
use cmd_parser::CmdParser;

#[tokio::main]
async fn main() {
    let _ = env_logger::builder()
        .is_test(false) // To enable color. Logs are not captured by test framework.
        .try_init();

    let cmd_parser = CmdParser::new();
    let dest = cmd_parser.dest().unwrap();
    let input = cmd_parser.logs().unwrap();

    let mut agent = Agent::new(dest).unwrap();
    for j in input {
        let j = j.unwrap();
        agent.write(j.to_string()).await.unwrap()
    }
}
