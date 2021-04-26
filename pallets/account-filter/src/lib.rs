//! # Account Filter Pallet
//!
//! The Account Filter Pallet provides functionality to restrict extrinsic submission to a set of
//! allowed accounts. The filtering of accounts is done during the transaction queue validation.

#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use frame_support::weights::DispatchInfo;
use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, SignedExtension},
    transaction_validity::{
        InvalidTransaction, TransactionLongevity, TransactionPriority, TransactionValidity,
        TransactionValidityError, ValidTransaction,
    },
};
use sp_std::fmt::Debug;
use sp_std::marker::PhantomData;
use sp_std::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    #[pallet::storage]
    pub type AllowedAccounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // When a new account is added to the allow-list.
        AccountAllowed(T::AccountId),
        // When an account is removed from the allow-list.
        AccountRemoved(T::AccountId),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub allowed_accounts: Vec<(T::AccountId, ())>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                allowed_accounts: Vec::new(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Pallet::<T>::initialize_allowed_accounts(&self.allowed_accounts);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new account to the allow-list.
        /// Can only be called by the root.
        #[pallet::weight(0)]
        pub fn add_account(
            origin: OriginFor<T>,
            new_account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            <AllowedAccounts<T>>::insert(&new_account, ());

            Self::deposit_event(Event::AccountAllowed(new_account));

            Ok(().into())
        }
        
        /// Remove an account from the allow-list.
        /// Can only be called by the root.
        #[pallet::weight(0)]
        pub fn remove_account(
            origin: OriginFor<T>,
            account_to_remove: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            <AllowedAccounts<T>>::remove(&account_to_remove);

            Self::deposit_event(Event::AccountRemoved(account_to_remove));

            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn initialize_allowed_accounts(allowed_accounts: &[(T::AccountId, ())]) {
        if !allowed_accounts.is_empty() {
            for (account, extrinsics) in allowed_accounts.iter() {
                <AllowedAccounts<T>>::insert(account, extrinsics);
            }
        }
    }
}

/// The following section implements the `SignedExtension` trait
/// for the `AllowAccount` type.
/// `SignedExtension` is being used here to filter out the not allowed accounts
/// when they try to send extrinsics to the runtime.
/// Inside the `validate` function of the `SignedExtension` trait,
/// we check if the sender (origin) of the extrinsic is part of the
/// allow-list or not.
/// The extrinsic will be rejected as invalid if the origin is not part
/// of the allow-list.
/// The validation happens at the transaction queue level,
///  and the extrinsics are filtered out before they hit the pallet logic.

/// The `AllowAccount` struct.
#[derive(Encode, Decode, Clone, Eq, PartialEq, Default)]
pub struct AllowAccount<T: Config + Send + Sync>(PhantomData<T>);

impl<T: Config + Send + Sync> AllowAccount<T> {
    /// utility constructor. Used only in client/factory code.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// Debug impl for the `AllowAccount` struct.
impl<T: Config + Send + Sync> Debug for AllowAccount<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "AllowAccount")
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

/// Implementation of the `SignedExtension` trait for the `AllowAccount` struct.
impl<T: Config + Send + Sync> SignedExtension for AllowAccount<T>
where
    T::Call: Dispatchable<Info = DispatchInfo>,
{
    type AccountId = T::AccountId;
    type Call = T::Call;
    type AdditionalSigned = ();
    type Pre = ();
    const IDENTIFIER: &'static str = "AllowAccount";

    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }

    // Filter out the not allowed keys.
    // If the key is in the allow-list, return a valid transaction,
    // else return a custom error.
    fn validate(
        &self,
        who: &Self::AccountId,
        _call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> TransactionValidity {
        if <AllowedAccounts<T>>::contains_key(who) {
            Ok(ValidTransaction {
                priority: info.weight as TransactionPriority,
                longevity: TransactionLongevity::max_value(),
                propagate: true,
                ..Default::default()
            })
        } else {
            Err(InvalidTransaction::Call.into())
        }
    }
}
