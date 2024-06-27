use crate::types::ProposalStatus;
use crate::{mock::*, Error, Proposals, Vote};
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};

fn create_proposal() {
	let proposal_name: Vec<u8> = "First Proposal".into();
	let bounded_proposal_name: BoundedVec<u8, ConstU32<20>> = proposal_name.try_into().unwrap();

	let proposal_description: Vec<u8> = "Description of first proposal test".into();
	let bounded_proposal_description: BoundedVec<u8, ConstU32<100>> =
		proposal_description.try_into().unwrap();

	assert_ok!(Proposal::create_proposal(
		RuntimeOrigin::signed(1),
		bounded_proposal_name,
		bounded_proposal_description,
		1
	));

	assert!(Proposals::<Test>::contains_key(1));
}

#[test]
fn create_proposal_works() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "First Proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<20>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of first proposal test".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<100>> =
			proposal_description.try_into().unwrap();

		assert_ok!(Proposal::create_proposal(
			RuntimeOrigin::signed(1),
			bounded_proposal_name,
			bounded_proposal_description,
			1
		));

		assert!(Proposals::<Test>::contains_key(1));
	});
}

#[test]
fn create_proposal_fails_invalid_proposal_duration() {
	new_test_ext().execute_with(|| {
		let proposal_name: Vec<u8> = "First Proposal".into();
		let bounded_proposal_name: BoundedVec<u8, ConstU32<20>> = proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of first proposal test".into();
		let bounded_proposal_description: BoundedVec<u8, ConstU32<100>> =
			proposal_description.try_into().unwrap();

		assert_noop!(
			Proposal::create_proposal(
				RuntimeOrigin::signed(1),
				bounded_proposal_name,
				bounded_proposal_description,
				0
			),
			Error::<Test>::InvalidProposalDuration
		);
	});
}

#[test]
fn vote_works() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert!(Proposals::<Test>::contains_key(1));

		assert_ok!(Proposal::vote(RuntimeOrigin::signed(2), 1, Vote::YES));

		assert!(Proposals::<Test>::get(1).unwrap().in_support.len() == 1);
	});
}

#[test]
fn vote_fails_proposer_votes() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert_noop!(
			Proposal::vote(RuntimeOrigin::signed(1), 1, Vote::YES),
			Error::<Test>::OwnerCannotVote
		);
	});
}

#[test]
fn vote_fails_proposal_not_exist() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert_noop!(
			Proposal::vote(RuntimeOrigin::signed(1), 2, Vote::YES),
			Error::<Test>::ProposalDoesNotExist
		);
	});
}

#[test]
fn vote_fails_proposal_not_active() {
	new_test_ext().execute_with(|| {
		create_proposal();

		run_to_block(15_000);
		assert_noop!(
			Proposal::vote(RuntimeOrigin::signed(1), 1, Vote::YES),
			Error::<Test>::ProposalNotActive
		);
	});
}

#[test]
fn vote_fails_duplicate_vote() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert_ok!(Proposal::vote(RuntimeOrigin::signed(2), 1, Vote::YES));

		assert_noop!(
			Proposal::vote(RuntimeOrigin::signed(2), 1, Vote::YES),
			Error::<Test>::DuplicateVote
		);
	});
}

#[test]
fn vote_fails_account_limit_reached() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert_ok!(Proposal::vote(RuntimeOrigin::signed(2), 1, Vote::YES));
		assert_ok!(Proposal::vote(RuntimeOrigin::signed(3), 1, Vote::YES));
		assert_ok!(Proposal::vote(RuntimeOrigin::signed(4), 1, Vote::YES));

		assert_noop!(
			Proposal::vote(RuntimeOrigin::signed(5), 1, Vote::YES),
			Error::<Test>::AccountLimitReached
		);
	});
}

#[test]
fn proposal_accepted() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert_ok!(Proposal::vote(RuntimeOrigin::signed(2), 1, Vote::YES));

		assert!(Proposals::<Test>::get(1).unwrap().in_support.len() == 1);
		assert!(Proposals::<Test>::get(1).unwrap().in_oppose.len() == 0);

		run_to_block(15_000);

		assert!(Proposals::<Test>::get(1).unwrap().status == ProposalStatus::Accepted);
	});
}

#[test]
fn proposal_rejected() {
	new_test_ext().execute_with(|| {
		create_proposal();

		assert_ok!(Proposal::vote(RuntimeOrigin::signed(2), 1, Vote::NO));

		assert!(Proposals::<Test>::get(1).unwrap().in_support.len() == 0);
		assert!(Proposals::<Test>::get(1).unwrap().in_oppose.len() == 1);

		run_to_block(20_000);

		assert!(Proposals::<Test>::get(1).unwrap().status == ProposalStatus::Rejected);
	});
}
