extern crate ethbot_core as core;
extern crate web3;

use core::Command;
use web3::futures::Future;
use web3::types::*;

pub fn run_commands(commands: &[Command]) {
	let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
	let web3 = web3::Web3::new(transport);

	for command in commands {
		match *command {
			Command::ShowAddressBalance(ref addr) => {
				let balance = web3.eth().balance((*addr.clone()).into(), None).wait().expect("Failed JSON-RPC request");
				println!("account '0x{:?}' balance: {}", addr, balance);
			},
			Command::ShowTransaction(ref tx_hash) => {
				let tx_hash: web3::types::H256 = (*tx_hash.clone()).into();

				let tx = web3.eth()
					.transaction(TransactionId::Hash(tx_hash))
					.wait()
					.expect("Failed JSON-RPC request");

				if let Some(tx) = tx {
					println!("transaction 0x{:?}: {:?}", tx_hash, tx);
				} else {
					let tx_receipt = web3.eth().transaction_receipt(tx_hash).wait().expect("Failed JSON-RPC request");
					if let Some(tx_receipt) = tx_receipt {
						println!("transaction receipt 0x{:?}: {:?}", tx_hash, tx_receipt);
					} else {
						println!("Nothing found");
					}
				}
			},
			Command::ShowBlock(number) => {
				let block = web3.eth()
					.block(BlockId::Number(BlockNumber::Number(number)))
					.wait()
					.expect("Failed JSON-RPC request");

				println!("block #{}: {:?}", number, block);
			}
		}
	}
}