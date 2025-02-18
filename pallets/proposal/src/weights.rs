
//! Autogenerated weights for pallet_proposal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Pankajs-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain=dev
// --wasm-execution=compiled
// --pallet=pallet-proposal
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/proposal/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_proposal.
pub trait WeightInfo {
	fn create_proposal() -> Weight;
	fn vote() -> Weight;
}

/// Weights for pallet_proposal using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Proposal::NextProposalId` (r:1 w:1)
	/// Proof: `Proposal::NextProposalId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Proposal::ProposalExpireTime` (r:0 w:1)
	/// Proof: `Proposal::ProposalExpireTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Proposal::Proposals` (r:0 w:1)
	/// Proof: `Proposal::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1527`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 1527)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Proposal::Proposals` (r:1 w:1)
	/// Proof: `Proposal::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `3631`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 3631)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Proposal::NextProposalId` (r:1 w:1)
	/// Proof: `Proposal::NextProposalId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Proposal::ProposalExpireTime` (r:0 w:1)
	/// Proof: `Proposal::ProposalExpireTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Proposal::Proposals` (r:0 w:1)
	/// Proof: `Proposal::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1527`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 1527)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Proposal::Proposals` (r:1 w:1)
	/// Proof: `Proposal::Proposals` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `3631`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 3631)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}