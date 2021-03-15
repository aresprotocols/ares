#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get};
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;
use sp_runtime::traits::Hash;
use sp_std::{
    collections::vec_deque::VecDeque,
    prelude::*,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    /// Period during which a request is valid
    type ValidityPeriod: Get<Self::BlockNumber>;

    /// The cache aggregate price queue num.
    type AggregateQueueNum: Get<u32>;

    /// The duration in which oracles should on chain aggregate result.
    type AggregateInterval: Get<Self::BlockNumber>;
}

// Uniquely identify a request's specification understood by an Aggregator
pub type TokenSpec = Vec<u8>;

/// Aggregator which is desc info.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
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
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
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
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
pub struct AggregateResult<BlockNumber> {
    /// Block number at the time aggregate is created..
    pub block_number: BlockNumber,
    /// on chain price
    pub price: u64,
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	trait Store for Module<T: Config> as AresModule {
		// A set of all registered Aggregator
		pub Aggregators get(fn aggregator): map hasher(blake2_128_concat) T::AccountId => Aggregator<T::AccountId, T::BlockNumber>;

		// A running counter used internally to identify the next request
		pub NextRequestId get(fn request_id): u64;

		// A map of details of each running request
		pub Requests get(fn requests): map hasher(blake2_128_concat) u64 => Request<T::AccountId, T::BlockNumber, T::Hash>;

		// A map of details of each oracle result
		pub OracleResults get(fn oracle_results): map hasher(blake2_128_concat) TokenSpec => Vec<u64>;

		// A map of details of on chain aggregator result
		pub AggregatorResults get(fn aggregator_results): map hasher(blake2_128_concat) TokenSpec => AggregateResult<T::BlockNumber>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		// A request has been accepted.
		OracleRequest(AccountId, TokenSpec, u64, AccountId, Vec<u8>, Vec<u8>),

		// A request has been answered.
		OracleResult(AccountId, u64, AccountId, u64),

		// on chain aggregator result.
		AggregatorResult(u64),

		// A new aggregator has been registered
		AggregatorRegistered(AccountId),

		// An existing aggregator has been unregistered
		AggregatorUnregistered(AccountId),

		// A request didn't receive any result in time
		RemoveRequest(u64),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
	    // Manipulating an unknown aggregator
		UnknownAggregator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected aggregator
		WrongAggregator,
		// An aggregator is already registered.
		AggregatorAlreadyRegistered,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		// Register a new Aggregator.
		// Fails with `AggregatorAlreadyRegistered` if this Aggregator (identified by `origin`) has already been registered.
		#[weight = 10_000]
		pub fn register_aggregator(origin, source: Vec<u8>, alias: Vec<u8>, url: Vec<u8>) -> dispatch::DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			ensure!(!<Aggregators<T>>::contains_key(who.clone()), Error::<T>::AggregatorAlreadyRegistered);

			let now = frame_system::Module::<T>::block_number();

			<Aggregators::<T>>::insert(&who, Aggregator {
          		  account_id: who.clone(),
          		  block_number: now,
         		  source,
          		  alias,
          		  url,
       		 });

			Self::deposit_event(RawEvent::AggregatorRegistered(who));

			Ok(())
		}

		// Unregisters an existing Aggregator
		#[weight = 10_000]
		pub fn unregister_aggregator(origin) -> dispatch::DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			ensure!(<Aggregators<T>>::contains_key(who.clone()), Error::<T>::UnknownAggregator);

			let aggregator = <Aggregators<T>>::take(who.clone());

			if who == aggregator.account_id {
				Self::deposit_event(RawEvent::AggregatorUnregistered(who));
				Ok(())
			} else {
				Err(Error::<T>::UnknownAggregator.into())
			}
		}

		// Identify oracle request from outside
		// spec_index mark btc or eth price
		#[weight = 10_000]
		pub fn initiate_request(origin, aggregator: T::AccountId, token: TokenSpec, data: Vec<u8>) -> dispatch::DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Aggregators<T>>::contains_key(aggregator.clone()), Error::<T>::UnknownAggregator);

			// Currently, one origin can only offload one computation per block. We should probably
			// include some nonce in the hash so this limitation is lifted.
			let request_id = NextRequestId::get();
			NextRequestId::put(request_id + 1);

			let work_id = (&who, <frame_system::Module<T>>::block_number(), request_id)
				.using_encoded(T::Hashing::hash);

			let now = frame_system::Module::<T>::block_number();
			Requests::<T>::insert(request_id.clone(), Request  {
				aggregator_id: aggregator.clone(),
				block_number: now,
				token: token.clone(),
				work_id,
			});

			Self::deposit_event(RawEvent::OracleRequest(aggregator, token, request_id, who, data, "Ares.callback".into()));

			Ok(())
		}

		// when aggregator get price from outside will callback token price
		#[weight = 10_000]
        fn feed_result(origin, request_id: u64, result: u64) -> dispatch::DispatchResult {
			//let _who = ensure_signed(origin)?;
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin.clone())?;

			ensure!(<Requests<T>>::contains_key(&request_id), Error::<T>::UnknownRequest);
			let aggregator = Self::requests(&request_id);

			ensure!(aggregator.aggregator_id == who, Error::<T>::WrongAggregator);

			let request = <Requests<T>>::take(request_id.clone());

			let mut buf: VecDeque<u64> = Self::oracle_results(&request.token).into_iter().collect();

			if buf.len() >= T::AggregateQueueNum::get() as usize {
				let _ = buf.pop_front();
			}
			buf.push_back(result);
			debug::info!("Number vector: {:?}", result);

			let price_vec: Vec<u64> = buf.into_iter().collect();

			OracleResults::insert(&request.token,price_vec);

			Self::deposit_event(RawEvent::OracleResult(request.aggregator_id, request_id, who, result));

			Ok(())
		}

		/// Identify requests that are considered dead and remove them
		/// on chain aggregator result
		fn on_finalize(n: T::BlockNumber) {
			for (request_identifier, request) in Requests::<T>::iter() {
				if n > request.block_number + T::ValidityPeriod::get() {
					// No result has been received in time
					Requests::<T>::remove(request_identifier);

					Self::deposit_event(RawEvent::RemoveRequest(request_identifier));
				}
			}

			for (token_identifier, price_vec) in OracleResults::iter() {

				let mut price: u64 = 0;
				let mut find: bool = false;

				if <AggregatorResults<T>>::contains_key(&token_identifier) {

					if price_vec.len() >= T::AggregateQueueNum::get() as usize {

						let request = Self::aggregator_results(&token_identifier);

						if n > request.block_number + T::AggregateInterval::get() {
							// update
							 price = Self::average_price(price_vec);
							 find = true;
						}
					}
				} else {
					price = Self::average_price(price_vec);
					find = true;
				}

				if find {
						<AggregatorResults::<T>>::insert(&token_identifier, AggregateResult {
							block_number: n,
							price,
						});

						Self::deposit_event(RawEvent::AggregatorResult(price));
				}

			}

		}

	 }
}

impl<T: Config> Module<T> {
    pub fn get_aggrage_price(_token_identifier:Vec<u8>) -> u64 {
        let mut price: u64 = 0;
        if <AggregatorResults<T>>::contains_key(&_token_identifier) {
            let aggragate = Self::aggregator_results(_token_identifier);
            // let price=aggragate::price;
            return aggragate.price;
        } else {
            0
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
}