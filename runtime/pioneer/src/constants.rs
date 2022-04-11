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

	/// Since BABE is probabilistic this is the average expected block time that
	/// we are targeting. Blocks will be produced at a minimum duration defined
	/// by `SLOT_DURATION`, but some slots will not be allocated to any
	/// authority and hence no block will be produced. We expect to have this
	/// block time on average following the defined slot duration and the value
	/// of `c` configured for BABE (where `1 - c` represents the probability of
	/// a slot being empty).
	/// This value is only used indirectly to define the unit constants below
	/// that are expressed in blocks. The rest of the code should use
	/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
	/// minimum period).
	///
	/// If using BABE with secondary slots (default) then all of the slots will
	/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
	/// `SLOT_DURATION` should have the same value.
	///
	/// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 2 * HOURS;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
}
