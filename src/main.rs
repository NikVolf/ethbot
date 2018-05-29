extern crate ethbot_core as core;
extern crate ethbot_runner as runner;

use core::Query;
use runner::run_commands;

fn main() {
	let commands = core::process_query(Query::Plain(::std::env::args().nth(1).expect("Usage: ethbot <text>")));
	run_commands(&commands[..]);
}
