#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::Erc20;
use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_prelude::string::String;

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        InsufficientSupply,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct Erc20 {}

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(
            _initial_supply: Balance,
            _name: Option<String>,
            _symbol: Option<String>,
            _decimals: Option<u8>,
        ) -> Self {
            unimplemented!()
        }

        /// Returns the token name.
        #[ink(message, selector = "0x6b1bb951")]
        pub fn token_name(&self) -> Option<String> {
            unimplemented!()
        }

        /// Returns the token symbol.
        #[ink(message, selector = "0xb42c3368")]
        pub fn token_symbol(&self) -> Option<String> {
            unimplemented!()
        }

        /// Returns the token decimals.
        #[ink(message, selector = "0xc64b0eb2")]
        pub fn token_decimals(&self) -> Option<u8> {
            unimplemented!()
        }

        /// Returns the total token supply.
        #[ink(message, selector = "0x143862ae")]
        pub fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message, selector = "0xb7d968c9")]
        pub fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message, selector = "0x10d455c2")]
        pub fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message, selector = "0xc04aa300")]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account balance of `from`.
        #[ink(message, selector = "0xbb399017")]
        pub fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<()> {
            unimplemented!()
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message, selector = "0x4ce0e831")]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Issue a new amount of tokens
        /// these tokens are deposited into the owner address
        #[ink(message)]
        pub fn mint(&mut self, _user: AccountId, _amount: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Redeem tokens.
        /// These tokens are withdrawn from the owner address
        /// if the balance must be enough to cover the redeem
        /// or the call will fail.
        #[ink(message)]
        pub fn burn(&mut self, _user: AccountId, _amount: Balance) -> Result<()> {
            unimplemented!()
        }
    }
}
