#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{debug, decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_system::offchain::{
    AppCrypto, CreateSignedTransaction, SendSignedTransaction,
    SignedPayload, Signer, SigningTypes,
};
use lite_json::json::JsonValue;
use sp_core::crypto::KeyTypeId;
use sp_runtime::offchain::Duration;
use sp_runtime::offchain::http;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::RuntimeDebug;
use sp_std::{
    collections::vec_deque::VecDeque,
    prelude::*,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    type Call: From<Call<Self>>;
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

    /// A grace period after we send transaction.
    ///
    /// To avoid sending too many transactions, we only attempt to send one
    /// every `GRACE_PERIOD` blocks. We use Local Storage to coordinate
    /// sending between distinct runs of this offchain worker.
    type GracePeriod: Get<Self::BlockNumber>;
}

/// Payload used by this example crate to hold price
/// data required to submit a transaction.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct PricePayload<Public, BlockNumber> {
    block_number: BlockNumber,
    price: u32,
    public: Public,
}

impl<T: SigningTypes> SignedPayload<T> for PricePayload<T::Public, T::BlockNumber> {
    fn public(&self) -> T::Public {
        self.public.clone()
    }
}

// The key type ID can be any 4-character string
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");
pub const NUM_VEC_LEN: usize = 20;

pub mod crypto {
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        MultiSignature,
        MultiSigner, traits::Verify,
    };
    use sp_runtime::app_crypto::{app_crypto, sr25519};

    use crate::KEY_TYPE;

    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

    // implemented for ocw-runtime
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
    for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}
// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		/// A vector of recently submitted prices. Bounded by NUM_VEC_LEN
		///
		/// This is used to calculate average price, should have bounded size.
		Prices get(fn prices): VecDeque<u32>;
		/// Defines the block when next unsigned transaction will be accepted.
		///
		/// To prevent spam of unsigned (and unpayed!) transactions on the network,
		/// we only allow one transaction every `T::UnsignedInterval` blocks.
		/// This storage entry defines when new transaction is going to be accepted.
		NextUnsignedAt get(fn next_unsigned_at): T::BlockNumber;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		NewPrice(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
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

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}

		#[weight = 10_000]
		pub fn submit_price(origin, price: u32) -> dispatch::DispatchResult {
			// Retrieve sender of the transaction.
			let who = ensure_signed(origin)?;
			// Add the price to the on-chain list.
			Self::add_price(who, price);
			Ok(())
		}

    fn offchain_worker(block_number: T::BlockNumber) {
    	let start_type = block_number % 5u32.into();
		if start_type == T::BlockNumber::from(1u32) {
			// It's a good idea to add logs to your offchain workers.
			// Using the `frame_support::debug` module you have access to the same API exposed by
			// the `log` crate.
			// Note that having logs compiled to WASM may cause the size of the blob to increase
			// significantly. You can use `RuntimeDebug` custom derive to hide details of the types
			// in WASM or use `debug::native` namespace to produce logs only when the worker is
			// running natively.
			debug::native::info!("Ares offchain workers! {} ",block_number);

			// Since off-chain workers are just part of the runtime code, they have direct access
			// to the storage and other included pallets.
			//
			// We can easily import `frame_system` and retrieve a block hash of the parent block.
			let parent_hash = <frame_system::Module<T>>::block_hash(block_number - 1u32.into());
			debug::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);

			// It's a good practice to keep `fn offchain_worker()` function minimal, and move most
			// of the code to separate `impl` block.
			// Here we call a helper function to calculate current average price.
			// This function reads storage entries of the current state.
			let average: Option<u32> = Self::average_price();
			debug::debug!("Current price: {:?}", average);

			// For this example we are going to send both signed and unsigned transactions
			// depending on the block number.
			// Usually it's enough to choose one or the other.
			let should_send = Self::choose_transaction_type(block_number);
			let res = match should_send {
				TransactionType::Signed => Self::fetch_price_and_send_signed(),
				TransactionType::None => Ok(()),
			};
			if let Err(e) = res {
				debug::error!("Error: {}", e);
			}
		}
    }
	}
}


enum TransactionType {
    Signed,
    None,
}

/// Most of the functions are moved outside of the `decl_module!` macro.
///
/// This greatly helps with error messages, as the ones inside the macro
/// can sometimes be hard to debug.
impl<T: Config> Module<T> {
    /// Chooses which transaction type to send.
    ///
    /// This function serves mostly to showcase `StorageValue` helper
    /// and local storage usage.
    ///
    /// Returns a type of transaction that should be produced in current run.
    fn choose_transaction_type(block_number: T::BlockNumber) -> TransactionType {
        /// A friendlier name for the error that is going to be returned in case we are in the grace
        /// period.
        const RECENTLY_SENT: () = ();

        // Start off by creating a reference to Local Storage value.
        // Since the local storage is common for all offchain workers, it's a good practice
        // to prepend your entry with the module name.
        let val = StorageValueRef::persistent(b"example_ocw::last_send");
        // The Local Storage is persisted and shared between runs of the offchain workers,
        // and offchain workers may run concurrently. We can use the `mutate` function, to
        // write a storage entry in an atomic fashion. Under the hood it uses `compare_and_set`
        // low-level method of local storage API, which means that only one worker
        // will be able to "acquire a lock" and send a transaction if multiple workers
        // happen to be executed concurrently.
        let res = val.mutate(|last_send: Option<Option<T::BlockNumber>>| {
            // We match on the value decoded from the storage. The first `Option`
            // indicates if the value was present in the storage at all,
            // the second (inner) `Option` indicates if the value was succesfuly
            // decoded to expected type (`T::BlockNumber` in our case).
            match last_send {
                // If we already have a value in storage and the block number is recent enough
                // we avoid sending another transaction at this time.
                Some(Some(block)) if block_number < block + T::GracePeriod::get() => {
                    Err(RECENTLY_SENT)
                }
                // In every other case we attempt to acquire the lock and send a transaction.
                _ => Ok(block_number)
            }
        });

        // The result of `mutate` call will give us a nested `Result` type.
        // The first one matches the return of the closure passed to `mutate`, i.e.
        // if we return `Err` from the closure, we get an `Err` here.
        // In case we return `Ok`, here we will have another (inner) `Result` that indicates
        // if the value has been set to the storage correctly - i.e. if it wasn't
        // written to in the meantime.
        match res {
            // The value has been set correctly, which means we can safely send a transaction now.
            Ok(Ok(block_number)) => {
                // Depending if the block is even or odd we will send a `Signed` or `Unsigned`
                // transaction.
                // Note that this logic doesn't really guarantee that the transactions will be sent
                // in an alternating fashion (i.e. fairly distributed). Depending on the execution
                // order and lock acquisition, we may end up for instance sending two `Signed`
                // transactions in a row. If a strict order is desired, it's better to use
                // the storage entry for that. (for instance store both block number and a flag
                // indicating the type of next transaction to send).
                let _transaction_type = block_number % 3u32.into();
                TransactionType::Signed
            }
            // We are in the grace period, we should not send a transaction this time.
            Err(RECENTLY_SENT) => TransactionType::None,
            // We wanted to send a transaction, but failed to write the block number (acquire a
            // lock). This indicates that another offchain worker that was running concurrently
            // most likely executed the same logic and succeeded at writing to storage.
            // Thus we don't really want to send the transaction, knowing that the other run
            // already did.
            Ok(Err(_)) => TransactionType::None,
        }
    }

    /// A helper function to fetch the price and send signed transaction.
    fn fetch_price_and_send_signed() -> Result<(), &'static str> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC."
            )?;
        }
        // Make an external HTTP request to fetch the current price.
        // Note this call will block until response is received.
        let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;

        // Using `send_signed_transaction` associated type we create and submit a transaction
        // representing the call, we've just created.
        // Submit signed will return a vector of results for all accounts that were found in the
        // local keystore with expected `KEY_TYPE`.
        let results = signer.send_signed_transaction(
            |_account| {
                // Received price is wrapped into a call to `submit_price` public function of this pallet.
                // This means that the transaction, when executed, will simply call that function passing
                // `price` as an argument.
                Call::submit_price(price)
            }
        );

        for (acc, res) in &results {
            match res {
                Ok(()) => debug::info!("[{:?}] Submitted price of {} cents", acc.id, price),
                Err(e) => debug::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
            }
        }

        Ok(())
    }

    /// Fetch current price and return the result in cents.
    fn fetch_price() -> Result<u32, http::Error> {
        // We want to keep the offchain worker execution time reasonable, so we set a hard-coded
        // deadline to 2s to complete the external call.
        // You can also wait idefinitely for the response, however you may still get a timeout
        // coming from the host machine.
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        // Initiate an external HTTP GET request.
        // This is using high-level wrappers from `sp_runtime`, for the low-level calls that
        // you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
        // since we are running in a custom WASM execution environment we can't simply
        // import the library here.
        let request = http::Request::get(
            "http://141.164.45.97:8080/ares/api/getPartyPrice/btcusdt"
        );
        // We set the deadline for sending of the request, note that awaiting response can
        // have a separate deadline. Next we send the request, before that it's also possible
        // to alter request headers or stream body content in case of non-GET requests.
        let pending = request
            .deadline(deadline)
            .send()
            .map_err(|_| http::Error::IoError)?;

        // The request is already being processed by the host, we are free to do anything
        // else in the worker (we can send multiple concurrent requests too).
        // At some point however we probably want to check the response though,
        // so we can block current thread and wait for it to finish.
        // Note that since the request is being driven by the host, we don't have to wait
        // for the request to have it complete, we will just not read the response.
        let response = pending.try_wait(deadline)
            .map_err(|_| http::Error::DeadlineReached)??;
        // Let's check the status code before we proceed to reading the response.
        if response.code != 200 {
            debug::warn!("Unexpected status code: {}", response.code);
            return Err(http::Error::Unknown);
        }

        // Next we want to fully read the response body and collect it to a vector of bytes.
        // Note that the return object allows you to read the body in chunks as well
        // with a way to control the deadline.
        let body = response.body().collect::<Vec<u8>>();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            debug::warn!("No UTF8 body");
            http::Error::Unknown
        })?;

        let price = match Self::parse_price(body_str) {
            Some(price) => Ok(price),
            None => {
                debug::warn!("Unable to extract price from the response: {:?}", body_str);
                Err(http::Error::Unknown)
            }
        }?;

        debug::warn!("Got price: {} cents", price);

        Ok(price)
    }

    /// Parse the price from the given JSON string using `lite-json`.
    ///
    /// Returns `None` when parsing failed or `Some(price in cents)` when parsing is successful.
    fn parse_price(price_str: &str) -> Option<u32> {
        let val = lite_json::parse_json(price_str);
        let price = val.ok().and_then(|v| match v {
            JsonValue::Object(obj) => {
                obj.into_iter()
                    .find(|(k, _)| {
                        let mut chars = "data".chars();
                        k.iter().all(|k| Some(*k) == chars.next())
                    })
                    .and_then(|v|
                        match v.1 {
                            JsonValue::Object(obj) => {
                                obj.into_iter()
                                    .find(|(k, _)| {
                                        let mut chars = "price".chars();
                                        k.iter().all(|k| Some(*k) == chars.next())
                                    })
                                    .and_then(|v| match v.1 {
                                        JsonValue::Number(number) => Some(number),
                                        _ => None,
                                    })
                            }
                            _ => None,
                        }
                    )
            }
            _ => None
        })?;

        let exp = price.fraction_length.checked_sub(2).unwrap_or(0);
        Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
    }

    /// Add new price to the list.
    fn add_price(who: T::AccountId, price: u32) {
        debug::info!("Adding to the average: {}", price);
        Prices::mutate(|prices| {
            if prices.len() == NUM_VEC_LEN {
                let _ = prices.pop_front();
            }
            prices.push_back(price);
            debug::info!("Number vector: {:?}", prices);
        });

        let average = Self::average_price()
            .expect("The average is not empty, because it was just mutated; qed");
        debug::info!("Current average price is: {}", average);
        // here we are raising the NewPrice event
        Self::deposit_event(RawEvent::NewPrice(price, who));
    }

    /// Calculate current average price.
    /// sort price and remove start , end value
    fn average_price() -> Option<u32> {
        let prices = Prices::get();
        if prices.is_empty() {
            None
        } else if prices.len() <= 2 {
            Some(prices.iter().fold(0_u32, |a, b| a.saturating_add(*b)) / prices.len() as u32)
        } else {
            let mut prices_ap: Vec<u32> = prices.into_iter().clone().collect();

            prices_ap.sort();
            prices_ap.truncate(prices_ap.len() - 1);
            let rest: Vec<u32> = prices_ap.drain(1..).collect();

            Some(rest.iter().fold(0_u32, |a, b| a.saturating_add(*b)) / rest.len() as u32)
        }
    }
}