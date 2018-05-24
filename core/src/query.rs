
#[derive(PartialEq, Eq, Debug)]
pub enum Query {
	Plain(String),
	Transaction(String),
	Address(String),
}