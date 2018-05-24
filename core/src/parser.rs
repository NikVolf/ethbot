use parity_hash::{H256, Address};

use {Command, Query};

fn without_0x(text: &str) -> &str {
	if text.starts_with("0x") { &text[2..] } else { text }
}

pub fn process_query(query: Query) -> Vec<Command> {
	let mut commands = Vec::new();
	match query {
		Query::Plain(text) => {
			if text.len() == 64 || text.len() == 66 {
				let hash: H256 = text.parse().expect("Failed to parse hash");
				commands.push(Command::ShowTransaction(hash));
			} else if text.len() == 40 || text.len() == 42 {
				let hash: Address = without_0x(&text).parse().expect("Failed to parse hash");
				commands.push(Command::ShowAddressBalance(hash));
			}
		},
		Query::Address(text) => {
			let hash: Address = without_0x(&text).parse().expect("Failed to parse hash");
			commands.push(Command::ShowAddressBalance(hash));
		},
		Query::Transaction(text) => {
			let hash: H256 = text.parse().expect("Failed to parse hash");
			commands.push(Command::ShowTransaction(hash));
		}
	}
	commands
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn address() {
		assert_eq!(
			process_query(Query::Address("0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f".to_owned())),
			vec![
				Command::ShowAddressBalance("0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f".parse().unwrap())
			]
		);
	}

	#[test]
	fn ox_address() {
		assert_eq!(
			process_query(Query::Address("0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f".to_owned())),
			vec![
				Command::ShowAddressBalance("0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f".parse().unwrap())
			]
		);
	}
}