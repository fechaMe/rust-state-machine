use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

// #[cfg(test)]
// mod test {
// 	#[test]
// 	fn init_system() {
// 		let mut system = super::Pallet::<String, u32, u32>::new();
// 		system.inc_block_number();
// 		system.inc_nonce(&"alice".to_string());
// 	}
// }
