pub mod currency {
	pub use runtime_common::Balance;

	/// The existential deposit. Set to 1/10 of its parent Relay Chain.
	// pub const EXISTENTIAL_DEPOSIT: Balance = 100 * MILLICENTS;
	pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

	// pub const MILLICENTS: Balance = 1_000_000_000;
	// pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
	// pub const DOLLARS: Balance = 100 * CENTS;

	pub const DOLLARS: Balance = 1_000_000_000_000;
	pub const CENTS: Balance = DOLLARS / 100;
	pub const GRAND: Balance = CENTS * 100_000;
	pub const MILLICENTS: Balance = CENTS / 1_000;
	pub const ARES_AMOUNT_MULT: Balance = 1;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 2_000 * CENTS + (bytes as Balance) * 100 * MILLICENTS
	}
}

/// Time.
pub mod time {
	/// An index to a block.
	pub use runtime_common::BlockNumber;
	/// Type used for expressing timestamp.
	pub type Moment = u64;

	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000; // take 6

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 1 * HOURS;
	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

/// Fee-related.
pub mod fee {
	use frame_support::weights::{WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial};
	use runtime_common::{Balance, ExtrinsicBaseWeight};
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, `MAXIMUM_BLOCK_WEIGHT`]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Kusama, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
			let p = super::currency::CENTS;
			let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}
