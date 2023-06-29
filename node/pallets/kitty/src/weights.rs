#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
	fn create_kitty() -> Weight;
	fn transfer() -> Weight;
	fn set_price() -> Weight;
	fn buy_kitty() -> Weight;
}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_kitty() -> Weight {
		todo!()
	}

	fn transfer() -> Weight {
		todo!()
	}

	fn set_price() -> Weight {
		todo!()
	}

	fn buy_kitty() -> Weight {
		todo!()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_kitty() -> Weight {
		todo!()
	}

	fn transfer() -> Weight {
		todo!()
	}

	fn set_price() -> Weight {
		todo!()
	}

	fn buy_kitty() -> Weight {
		todo!()
	}
}
