use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Default)]
#[scale_info(skip_type_params(NameLimit, DescriptionLimit, AccountLimit))]
pub struct Proposal<
	AccountId,
	NameLimit: Get<u32>,
	DescriptionLimit: Get<u32>,
	AccountLimit: Get<u32>,
> {
	pub proposer: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub status: bool,
	pub voter_accounts: BoundedVec<AccountId, AccountLimit>,
	pub in_support: BoundedVec<AccountId, AccountLimit>,
	pub in_oppose: BoundedVec<AccountId, AccountLimit>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug)]
pub enum Vote {
	YES,
	NO,
}

/// Result of proposal.
#[derive(Eq, PartialEq, Clone, TypeInfo, Encode, Decode)]
pub enum ProposalResultStatus {
	/// Proposal is passed.
	Accepted,
	/// Proposal is rejected.
	Rejected,
}
