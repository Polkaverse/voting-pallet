// Constants file.

// What should be the minimum duration for a proposal in days.
pub const PROPOSAL_DURATION_LIMIT: u32 = 1;

/// Blocks per day is a assumption of block generating by chain in 24 hours
/// Assuming chain generating the blocks in every 6 second. 1 Block = 6 second
pub const BLOCKS_PER_DAY: u32 = 10;
