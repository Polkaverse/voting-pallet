# Decentralized Voting System

This project contains the decentralized voting system. 
In which user can create the proposal on the chain and the other users of the chain can vote on the proposal for the given deadline.
After that if majority supports the proposal then proposal Accepted after deadline other-wise proposal rejected.

### Components of the Decentralized Voting System
#### Proposal Creation:

* Users can create new proposals by submitting a description and a duration for the voting period.
* Proposals are stored on-chain with a unique identifier, the creator's address, the description, and the voting period.

#### Voting:
* Users can cast votes on active proposals. 
* Each user can only vote once per proposal. 
* Votes can be either "Yes" or "No".

#### Tallying Votes:

* After the voting period (deadline) ends, votes are tallied.
* If the majority supports the proposal, it is accepted; otherwise, it is rejected.


# 🏆 Requirements

- Prepare your development environment ([Instructions](https://docs.substrate.io/install/))
- Clone the repository:

```
git clone https://github.com/Polkaverse/voting-pallet.git
```

# ⛳ Getting Started

Use this **QuickStart** command to build and launch the node:

### Build the Code

```bash
cargo run --release
```

### Run the Single Node in dev environment

```
./target/release/node-template --dev
```
