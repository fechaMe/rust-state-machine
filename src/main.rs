mod balances;
mod support;
mod system;

pub enum RuntimeCall {}

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
		Ok(())
	}
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice".to_string(), 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&"alice".to_string());
	let _res = runtime
		.balances
		.transfer(&"alice".to_string(), &"bob".to_string(), 30)
		.map_err(|e| eprintln!("{e}"));

	runtime.system.inc_nonce(&"alice".to_string());
	let _res = runtime
		.balances
		.transfer(&"alice".to_string(), &"charlie".to_string(), 20)
		.map_err(|e| eprintln!("{e}"));

	println!("{runtime:#?}")
}
