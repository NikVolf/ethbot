use parity_hash::{H256, Address};

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
	ShowTransaction(H256),
	ShowAddressBalance(Address),
}