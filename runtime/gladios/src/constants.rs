// pub mod currency {
// 	pub use runtime_common::Balance;
//
// 	/// The existential deposit. Set to 1/10 of its parent Relay Chain.
// 	pub const EXISTENTIAL_DEPOSIT: Balance = 100 * MILLICENTS;
//
// 	pub const MILLICENTS: Balance = 1_000_000_000;
// 	pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
// 	pub const DOLLARS: Balance = 100 * CENTS;
//
// 	pub const fn deposit(items: u32, bytes: u32) -> Balance {
// 		items as Balance * 10 * DOLLARS + (bytes as Balance) * 50 * MILLICENTS
// 	}
// }

pub mod currency {
	pub use runtime_common::Balance;

	/// The existential deposit. Set to 1/10 of its parent Relay Chain.
	pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

	pub const DOLLARS: Balance = 1_000_000_000_000;
	pub const CENTS: Balance = DOLLARS / 100;
	pub const GRAND: Balance = CENTS * 100_000;
	pub const MILLICENTS: Balance = CENTS / 1_000;
	pub const ARES_AMOUNT_MULT: Balance = 1;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
	}
}

/// Time.
pub mod time {
	/// An index to a block.
	pub type BlockNumber = u32;
	/// Type used for expressing timestamp.
	pub type Moment = u64;
	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
}

