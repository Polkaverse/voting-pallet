#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::{Proposal, ProposalStatus, Vote};
use frame_support::{dispatch::DispatchResultWithPostInfo, BoundedVec};
use sp_std::vec::Vec;

mod constants;
use crate::constants::{BLOCKS_PER_DAY, PROPOSAL_DURATION_LIMIT};
use frame_support::traits::Incrementable;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the Proposal.
		type ProposalId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// The maximum length of proposal name/title.
		#[pallet::constant]
		type NameLimit: Get<u32>;

		/// The maximum length of proposal description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		/// The maximum length of address.
		#[pallet::constant]
		type AccountLimit: Get<u32>;

		// Weight information
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store new proposal with a unique proposal id for a particular community
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::ProposalId,
		Proposal<
			T::AccountId,
			<T as pallet::Config>::NameLimit,
			<T as Config>::DescriptionLimit,
			T::AccountLimit,
			ProposalStatus,
		>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn proposal_expire)]
	pub type ProposalExpireTime<T: Config> =
		StorageMap<_, Identity, BlockNumberFor<T>, T::ProposalId, OptionQuery>;

	/// Stores the `ProposalId` that is going to be used for the next proposal.
	/// This gets incremented whenever a new proposal is created.
	#[pallet::storage]
	pub(super) type NextProposalId<T: Config> = StorageValue<_, T::ProposalId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Proposals [Proposal Id]
		CreatedProposal(T::ProposalId),
		/// Submitted Proposal [Proposal Id]
		VoteCasted(T::ProposalId),
		/// Proposal closed [Proposal Id]
		ProposalClosed(T::ProposalId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposal Does Not Exist.
		ProposalDoesNotExist,
		/// Invalid description given.
		BadDescription,
		/// Proposal got inactive.
		ProposalNotActive,
		/// Duplicate vote.
		DuplicateVote,
		/// New account can't be added due to account limit.
		AccountLimitReached,
		/// Invalid Proposal duration.
		InvalidProposalDuration,
		/// Proposal owner cannot vote on proposal.
		OwnerCannotVote,
		/// If creation of new bounded vector is not possible
		CannotBeBounded,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
			let option_proposal_expire = ProposalExpireTime::<T>::get(block_number);

            // There is any proposal deadline ending on this block then this will Announce the Result.
			if let Some(proposal_id) = option_proposal_expire {
				Proposals::<T>::try_mutate(proposal_id, |proposal_detail| -> DispatchResult {
					let proposal_data = proposal_detail
						.as_mut()
						.ok_or(Error::<T>::ProposalDoesNotExist)?;

					// fetching the vote information of the proposal.
					let support = &proposal_data.in_support.len();
					let oppose = &proposal_data.in_oppose.len();

					// Inserting the proposal result according to the voting.
					// If support is more than the oppose.
					if support > oppose {
						proposal_data.status = ProposalStatus::Accepted;
					} else {
						proposal_data.status = ProposalStatus::Rejected;
					};

					proposal_data.is_active = false;

					Self::deposit_event(Event::<T>::ProposalClosed(proposal_id));

					Ok(())
				})
				.expect("Proposal not found");
			}
			Weight::zero()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new proposal on the chain.
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the transaction, must be a signed account.
		/// * `name` - A bounded vector containing the name of the proposal.
		/// * `description` - A bounded vector containing the description of the proposal.
		/// * `proposal_duration` - The duration for which the proposal will be open for voting,
		///   specified in terms of a number of days. This value must be within the range defined
		///   by `PROPOSAL_DURATION_LIMIT`.
		///
		/// # Errors
		///
		/// * `Error::<T>::InvalidProposalDuration` - Returned if the specified proposal duration
		///   is not within the valid range defined by `PROPOSAL_DURATION_LIMIT`.
		///
		/// On successfully completion of method CreatedProposal Event will Emit.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::create_proposal())]
		pub fn create_proposal(
			origin: OriginFor<T>,
			name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
			description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
			proposal_duration: u32,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;

			// The given duration should be in the range from 1 to the given Duration limit.
			ensure!(
				(1..=PROPOSAL_DURATION_LIMIT).contains(&proposal_duration),
				Error::<T>::InvalidProposalDuration
			);

			Self::do_create_proposal(origin, name, description, proposal_duration)
		}

		/// Cast a vote on an existing proposal.
		///
		/// # Arguments
		///
		/// * `origin` - The origin of the transaction, must be a signed account.
		/// * `proposal_id` - The identifier of the proposal to vote on.
		/// * `choice` - The vote choice, which can be either `Vote::YES` or `Vote::NO`.
		///
		/// # Errors
		///
		/// * `Error::<T>::ProposalDoesNotExist` - Returned if the specified proposal does not exist.
		/// * `Error::<T>::ProposalNotActive` - Returned if the proposal is not active and cannot be voted on.
		/// * `Error::<T>::OwnerCannotVote` - Returned if the owner of the proposal attempts to vote on their own proposal.
		/// * `Error::<T>::DuplicateVote` - Returned if the account has already voted on the proposal.
		/// * `Error::<T>::AccountLimitReached` - Returned if the number of accounts voting on the proposal exceeds the allowed limit.
		///
		/// On successfully completion of method CreatedProposal Event will Emit.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::vote())]
		pub fn vote(
			origin: OriginFor<T>,
			proposal_id: T::ProposalId,
			choice: Vote,
		) -> DispatchResultWithPostInfo {
			let origin = ensure_signed(origin)?;

            // Proposal should exist on chain.
			let proposal =
				Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalDoesNotExist)?;

            // Proposal is exist or not.
			ensure!(proposal.is_active, Error::<T>::ProposalNotActive);

            // Proposal owner cannot vote on the proposal.
			ensure!(!(proposal.owner == origin), Error::<T>::OwnerCannotVote);

            // User should not vote multiple time on the proposal.
			ensure!(!(proposal.voter_accounts).contains(&origin), Error::<T>::DuplicateVote);

			// Add this account in voter_accounts list and respective vote option.
			Proposals::<T>::mutate(proposal_id, |proposal_details| -> DispatchResult {
				let proposal_info = proposal_details
					.as_mut()
					.ok_or(Error::<T>::ProposalDoesNotExist)?;

				proposal_info
					.voter_accounts
					.try_push(origin.clone())
					.ok()
					.ok_or(Error::<T>::AccountLimitReached)?;

				match choice {
					Vote::YES => {
						proposal_info
							.in_support
							.try_push(origin.clone())
							.ok()
							.ok_or(Error::<T>::AccountLimitReached)?;
						Ok(())
					},
					Vote::NO => {
						proposal_info
							.in_oppose
							.try_push(origin.clone())
							.ok()
							.ok_or(Error::<T>::AccountLimitReached)?;
						Ok(())
					},
				}
			})?;

			Self::deposit_event(Event::VoteCasted(proposal_id));
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_proposal(
		owner: T::AccountId,
		name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
		description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
		proposal_duration: u32,
	) -> DispatchResultWithPostInfo {
		let bounded_account: BoundedVec<T::AccountId, <T as Config>::AccountLimit> = Vec::new()
			.clone()
			.try_into()
			.map_err(|_| Error::<T>::CannotBeBounded)?;

		let new_proposal = Proposal {
			owner: owner.clone(),
			name,
			description,
			is_active: true,
			voter_accounts: bounded_account.clone(),
			in_support: bounded_account.clone(),
			in_oppose: bounded_account.clone(),
			status: ProposalStatus::VotingInProgress,
		};

		let proposal_id = NextProposalId::<T>::get().unwrap_or(
			T::ProposalId::initial_value()
				.expect("NOT FOUND")
				.increment()
				.expect("NOT FOUND"),
		);

		// Storing the proposal
		Proposals::<T>::insert(proposal_id, &new_proposal);

		// Set up the expire time of a particular proposal.
		let total_block: u32 = BLOCKS_PER_DAY * proposal_duration;

		let expire_block = frame_system::Pallet::<T>::block_number() + total_block.into();
		ProposalExpireTime::<T>::insert(expire_block, proposal_id);

        // Adding the proposal id for next proposal.
		let next_proposal_id = proposal_id.increment().expect("NOT FOUND");
		NextProposalId::<T>::set(Some(next_proposal_id));

		Self::deposit_event(Event::CreatedProposal(proposal_id));

		Ok(().into())
	}
}
