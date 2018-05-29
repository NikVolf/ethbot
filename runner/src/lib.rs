extern crate ethbot_core as core;
extern crate web3;

use core::Command;
use web3::futures::Future;

pub fn run_commands(commands: &[Command]) {
	let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
	let web3 = web3::Web3::new(transport);

	for command in commands {
		match *command {
			Command::ShowAddressBalance(ref addr) => {
				let balance = web3.eth().balance((*addr.clone()).into(), None).wait().expect("Failed JSON-RPC request");
				println!("account '0x{:?}' balance: {}", addr, balance);
			},
			_ => {

			}
		}
	}
}