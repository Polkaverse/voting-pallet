//! Benchmarking setup for pallet-proposal
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as ProposalPallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use crate::types::Proposal;

pub fn add_proposal<T: Config>(caller: T::AccountId) {
	let proposal_id = NextProposalId::<T>::get()
		.unwrap_or(T::ProposalId::initial_value().unwrap().increment().unwrap());

	let proposal_name: Vec<u8> = "First proposal".into();
	let bounded_proposal_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> =
		proposal_name.try_into().unwrap();

	let proposal_description: Vec<u8> = "Description of proposal".into();
	let bounded_proposal_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> =
		proposal_description.try_into().unwrap();

	let bounded_account: BoundedVec<T::AccountId, <T as Config>::AccountLimit> =
		Vec::new().clone().try_into().unwrap();

	let new_proposal = Proposal {
		owner: caller.clone(),
		name: bounded_proposal_name,
		description: bounded_proposal_description,
		is_active: true,
		voter_accounts: bounded_account.clone(),
		in_support: bounded_account.clone(),
		in_oppose: bounded_account.clone(),
		status: ProposalStatus::VotingInProgress,
	};

	<Proposals<T>>::insert(proposal_id, &new_proposal);
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_proposal() {
		let proposal_id = NextProposalId::<T>::get()
			.unwrap_or(T::ProposalId::initial_value().unwrap().increment().unwrap());

		let value = 1u32.into();
		let caller: T::AccountId = whitelisted_caller();

		let proposal_name: Vec<u8> = "First proposal".into();
		let bounded_proposal_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> =
			proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of proposal".into();
		let bounded_proposal_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> =
			proposal_description.try_into().unwrap();

		#[extrinsic_call]
		create_proposal(
			RawOrigin::Signed(caller.clone()),
			bounded_proposal_name,
			bounded_proposal_description,
			value,
		);

		assert_eq!(Proposals::<T>::get(proposal_id).unwrap().owner, caller);
	}

	#[benchmark]
	fn vote() {
		let proposal_id = NextProposalId::<T>::get()
			.unwrap_or(T::ProposalId::initial_value().unwrap().increment().unwrap());

		let caller: T::AccountId = whitelisted_caller();

		add_proposal::<T>(caller.clone());

		let voter: T::AccountId = account("sub", 1, 0);

		#[extrinsic_call]
		vote(RawOrigin::Signed(voter.clone()), proposal_id, Vote::YES);

		assert_eq!(Proposals::<T>::get(proposal_id).unwrap().owner, caller);
	}

	impl_benchmark_test_suite!(ProposalPallet, crate::mock::new_test_ext(), crate::mock::Test);
}
