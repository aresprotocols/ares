#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use sp_std::vec::Vec;
use codec::{Decode, Encode};
//use frame_support::{debug, dispatch, ensure, traits::Get};
//use frame_system::ensure_signed;
use frame_support::sp_runtime::RuntimeDebug;
//use sp_runtime::traits::Hash;
use frame_support::sp_runtime::traits::Hash;
pub type TokenSpec = Vec<u8>;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Aggregator which is desc info.
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, Eq, Default)]
pub struct Aggregator<AccountId, BlockNumber> {
	pub account_id: AccountId,
	/// Block number at the time register is created..
	pub block_number: BlockNumber,
	/// exchange source
	pub source: Vec<u8>,
	/// alias name
	pub alias: Vec<u8>,
	/// api url.
	pub url: Vec<u8>,
}

/// Requests which is quest info.
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, Eq, Default)]
pub struct Request<AccountId, BlockNumber, Hash> {
	pub aggregator_id: AccountId,
	/// Block number at the time request is created..
	pub block_number: BlockNumber,
	/// exchange source
	pub token: TokenSpec,
	/// chain work idzX
	pub work_id: Hash,
}

/// AggregateResult which is aggregate result.
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, Eq, Default)]
pub struct AggregateResult<BlockNumber> {
	/// Block number at the time aggregate is created..
	pub block_number: BlockNumber,
	/// on chain price
	pub price: u64,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; 
	use sp_std::{
		collections::vec_deque::VecDeque,
		prelude::*,
	};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	
		/// Period during which a request is valid
		type ValidityPeriod: Get<Self::BlockNumber>;

		/// The cache aggregate price queue num.
		type AggregateQueueNum: Get<u32>;
	
		/// The duration in which oracles should on chain aggregate result.
		type AggregateInterval: Get<Self::BlockNumber>;
	}

	// Uniquely identify a request's specification understood by an Aggregator
	
    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // A request has been accepted.
		OracleRequest(T::AccountId, TokenSpec, u64, T::AccountId, Vec<u8>, Vec<u8>),

		// A request has been answered.
		OracleResult(T::AccountId, u64, T::AccountId, u64),

		// on chain aggregator result.
		AggregatorResult(u64),

		// A new aggregator has been registered
		AggregatorRegistered(T::AccountId),

		// An existing aggregator has been unregistered
		AggregatorUnregistered(T::AccountId),

		// A request didn't receive any result in time
		RemoveRequest(u64),
    }

    #[pallet::error]   
	pub enum Error<T> {
		// Manipulating an unknown aggregator
		UnknownAggregator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected aggregator
		WrongAggregator,
		// An aggregator is already registered.
		AggregatorAlreadyRegistered,
	}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

	// A set of all registered Aggregator
    #[pallet::storage] 
	#[pallet::getter(fn aggregator)]
	pub type Aggregators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Aggregator<T::AccountId, T::BlockNumber>>;

	// A running counter used internally to identify the next request
	#[pallet::storage]
	#[pallet::getter(fn request_id)]
	pub type NextRequestId<T> = StorageValue<_, u64>;

	// A map of details of each running request
	#[pallet::storage]
	#[pallet::getter(fn requests)]
	pub type Requests<T: Config> = StorageMap<_, Blake2_128Concat, u64, Request<T::AccountId, T::BlockNumber, T::Hash>>;

	// A map of details of each oracle result
	#[pallet::storage]
	#[pallet::getter(fn oracle_results)]
	pub type OracleResults<T> = StorageMap<_, Blake2_128Concat, TokenSpec, Vec<u64>>;

	// A map of details of on chain aggregator result
	#[pallet::storage]
	#[pallet::getter(fn aggregator_results)]
	pub type AggregatorResults<T: Config> = StorageMap<_, Blake2_128Concat, TokenSpec, AggregateResult<T::BlockNumber>>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// Identify requests that are considered dead and remove them
		// on chain aggregator result
		fn on_finalize(n: T::BlockNumber) {
			for (request_identifier, request) in Requests::<T>::iter() {
				if n > request.block_number + T::ValidityPeriod::get() {
					// No result has been received in time
					Requests::<T>::remove(request_identifier);

					Self::deposit_event(Event::RemoveRequest(request_identifier));
				}
			}

			for (token_identifier, price_vec) in OracleResults::<T>::iter() {

				let mut price: u64 = 0;
				let mut find: bool = false;

				if <AggregatorResults<T>>::contains_key(&token_identifier) {

					if price_vec.len() >= T::AggregateQueueNum::get() as usize {

						let request = Self::aggregator_results(&token_identifier);

						if n > request.unwrap().block_number + T::AggregateInterval::get() {
							// update
							 price = average_price(price_vec);
							 find = true;
						}
					}
				} else {
					price = average_price(price_vec);
					find = true;
				}

				if find {
						<AggregatorResults::<T>>::insert(&token_identifier, AggregateResult {
							block_number: n,
							price,
						});

						Self::deposit_event(Event::AggregatorResult(price));
				}
			}
		}
	}

    #[pallet::call]   
	impl<T:Config> Pallet<T> {
		// Register a new Aggregator.
		// Fails with `AggregatorAlreadyRegistered` if this Aggregator (identified by `origin`) has already been registered.
		#[pallet::weight(10_000)]
		pub fn register_aggregator(origin: OriginFor<T>, source: Vec<u8>, alias: Vec<u8>, url: Vec<u8>) -> DispatchResultWithPostInfo {
			//fixe  let who : <T as frame_system::Trait>::AccountId  to let who
			let who = ensure_signed(origin)?;

			ensure!(!<Aggregators<T>>::contains_key(who.clone()), Error::<T>::AggregatorAlreadyRegistered);

			let now = frame_system::Pallet::<T>::block_number();

			<Aggregators::<T>>::insert(&who, Aggregator {
          		  account_id: who.clone(),
          		  block_number: now,
         		  source,
          		  alias,
          		  url,
       		 });

			Self::deposit_event(Event::AggregatorRegistered(who));

			Ok(().into())
		}

		// Unregisters an existing Aggregator
		#[pallet::weight(10_000)]
		pub fn unregister_aggregator(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(<Aggregators<T>>::contains_key(who.clone()), Error::<T>::UnknownAggregator);

			let aggregator = <Aggregators<T>>::take(who.clone()).expect("bad");

			if who == aggregator.account_id {
				Self::deposit_event(Event::AggregatorUnregistered(who));
				Ok(().into())
			} else {
				Err(Error::<T>::UnknownAggregator.into())
			}
		}

		// Identify oracle request from outside
		// spec_index mark btc or eth price
		#[pallet::weight(10_000)]
		pub fn initiate_request(origin: OriginFor<T>, aggregator: T::AccountId, token: TokenSpec, data: Vec<u8>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;

			ensure!(<Aggregators<T>>::contains_key(aggregator.clone()), Error::<T>::UnknownAggregator);

			// Currently, one origin can only offload one computation per block. We should probably
			// include some nonce in the hash so this limitation is lifted.
			//let request_id = 0;
			let request_id = match NextRequestId::<T>::get() {
				None => 0,
				Some(num) => num,
			};
			//let request_id = NextRequestId::<T>::get().expect("bad");
			NextRequestId::<T>::put(request_id + 1);

			let work_id = (&who, frame_system::Pallet::<T>::block_number(), request_id)
				.using_encoded(T::Hashing::hash);

			let now = frame_system::Pallet::<T>::block_number();
			Requests::<T>::insert(request_id.clone(), Request  {
				aggregator_id: aggregator.clone(),
				block_number: now,
				token: token.clone(),
				work_id,
			});

			Self::deposit_event(Event::OracleRequest(aggregator, token, request_id, who, data, "Ares.callback".into()));

			Ok(().into())
		}

		// when aggregator get price from outside will callback token price
		#[pallet::weight(10_000)]
        pub fn feed_result(origin: OriginFor<T>, request_id: u64, result: u64) -> DispatchResultWithPostInfo {
			//let _who = ensure_signed(origin)?;
			let who = ensure_signed(origin.clone())?;

			ensure!(<Requests<T>>::contains_key(&request_id), Error::<T>::UnknownRequest);
			let aggregator = Self::requests(&request_id);

			ensure!(aggregator.unwrap().aggregator_id == who, Error::<T>::WrongAggregator);
			let request = <Requests<T>>::take(request_id.clone()).expect("bad");
			
			//let mut buf : VecDeque<u64> = OracleResults::<T>::get(&request.token).expect("bad").into_iter().collect();
			let mut buf : VecDeque<u64> = match OracleResults::<T>::get(&request.token) {
				None => VecDeque::new(),
				Some(num) => num.into_iter().collect(),
			};

			if buf.len() >= T::AggregateQueueNum::get() as usize {
				let _ = buf.pop_front();
			}
			buf.push_back(result);
			//debug::info!("Number vector: {:?}", result);

			let price_vec: Vec<u64> = buf.into_iter().collect();

			OracleResults::<T>::insert(&request.token,price_vec);

			Self::deposit_event(Event::OracleResult(request.aggregator_id, request_id, who, result));

			Ok(().into())
		}
	}
}

fn average_price(prices: Vec<u64>) -> u64 {
    if prices.len() <= 2 {
        prices.iter().fold(0_u64, |a, b| a.saturating_add(*b)) / prices.len() as u64
    } else {
        let mut prices_ap: Vec<u64> = prices.into_iter().clone().collect();


        prices_ap.sort();
        prices_ap.truncate(prices_ap.len() - 1);
        let rest: Vec<u64> = prices_ap.drain(1..).collect();

        rest.iter().fold(0_u64, |a, b| a.saturating_add(*b)) / rest.len() as u64
    }
}

