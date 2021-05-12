#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::Erc20;
use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use erc20_trait::{Error as IError, IErc20, Result as IResult};
    use ink_prelude::string::String;
    use ownership::Ownable;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_lang as ink;
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{collections::HashMap as StorageHashMap, lazy::Lazy};

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        InsufficientSupply,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
        BlacklistedUser,
        InvalidAmount,
        OnlyOwnerAccess,
        InvalidNewOwner,
        NotBlacklistedUser,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Base contract which allows children to implement an emergency stop mechanism.
    #[ink::trait_definition]
    pub trait Pausable {
        /// Pause contract transaction.
        #[ink(message)]
        fn pause(&mut self) -> Result<()>;

        /// Recover paused contract.
        #[ink(message)]
        fn unpause(&mut self) -> Result<()>;

        /// Return contract pause statue.
        #[ink(message)]
        fn pause_state(&self) -> bool;
    }

    #[ink::trait_definition]
    pub trait BlackList {
        /// Whether the user is blacklisted.
        #[ink(message)]
        fn get_blacklist_status(&self, maker: AccountId) -> bool;

        /// Add illegal user to blacklist.
        #[ink(message)]
        fn add_blacklist(&mut self, evil_user: AccountId) -> Result<()>;

        /// Remove the user from blacklist.
        #[ink(message)]
        fn remove_blacklist(&mut self, cleared_user: AccountId) -> Result<()>;

        /// Destroy blacklisted user funds from total supply.
        #[ink(message)]
        fn destroy_blackfunds(&mut self, blacklisted_user: AccountId) -> Result<()>;
    }

    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Lazy<Balance>,
        /// Mapping from owner to number of owned token.
        balances: StorageHashMap<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        /// Name of the token
        name: Option<String>,
        /// Symbol of the token
        symbol: Option<String>,
        /// Decimals of the token
        decimals: Option<u8>,
        /// Implement an emergency stop mechanism.
        pause: bool,
        /// The contract owner, provides basic authorization control
        /// functions, this simplifies the implementation of "user permissions".
        owner: AccountId,

        blacklisted: StorageHashMap<AccountId, bool>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Pause {}

    #[ink(event)]
    pub struct Unpause {}

    #[ink(event)]
    pub struct DestroyedBlackFunds {
        #[ink(topic)]
        blacklisted_user: AccountId,
        #[ink(topic)]
        balance: Balance,
    }

    #[ink(event)]
    pub struct AddedBlackList {
        #[ink(topic)]
        user: AccountId,
    }

    #[ink(event)]
    pub struct RemovedBlackList {
        #[ink(topic)]
        user: AccountId,
    }

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl IErc20 for Erc20 {
        #[ink(constructor)]
        fn new(
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimals: Option<u8>,
        ) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, initial_supply);
            let instance = Self {
                total_supply: Lazy::new(initial_supply),
                balances,
                allowances: StorageHashMap::new(),
                name,
                symbol,
                decimals,
                pause: false,
                owner: caller,
                blacklisted: Default::default(),
            };
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
            instance
        }

        /// Returns the token name.
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }

        /// Returns the token symbol.
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }

        /// Returns the token decimals.
        #[ink(message)]
        fn token_decimals(&self) -> Option<u8> {
            self.decimals
        }

        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).copied().unwrap_or(0)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> IResult<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
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
        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> IResult<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(IError::InsufficientAllowance);
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert((from, caller), allowance - value);
            Ok(())
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> IResult<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }
    }

    impl Ownable for Erc20 {
        #[ink(constructor)]
        fn new() -> Self {
            unimplemented!()
        }

        /// Contract owner.
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            Some(self.owner)
        }

        /// transfer contract ownership to new owner.
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) {
            self.only_owner();
            if let Some(owner) = new_owner {
                self.owner = owner;
            }
        }
    }

    impl Pausable for Erc20 {
        /// Pause contract transaction.
        #[ink(message)]
        fn pause(&mut self) -> Result<()> {
            self.only_owner();

            if !self.pause {
                self.pause = true;
                self.env().emit_event(Pause {})
            }
            Ok(())
        }

        /// Recover paused contract.
        #[ink(message)]
        fn unpause(&mut self) -> Result<()> {
            self.only_owner();
            if self.pause {
                self.pause = false;
                self.env().emit_event(Unpause {})
            }
            Ok(())
        }

        /// Return contract pause statue.
        #[ink(message)]
        fn pause_state(&self) -> bool {
            self.pause
        }
    }

    impl BlackList for Erc20 {
        /// Whether the user is blacklisted.
        #[ink(message)]
        fn get_blacklist_status(&self, maker: AccountId) -> bool {
            self.blacklisted.get(&maker).copied().unwrap_or(false)
        }

        /// Add illegal user to blacklist.
        #[ink(message)]
        fn add_blacklist(&mut self, evil_user: AccountId) -> Result<()> {
            self.only_owner();
            self.blacklisted.insert(evil_user, true);
            Ok(())
        }

        /// Remove the user from blacklist.
        #[ink(message)]
        fn remove_blacklist(&mut self, cleared_user: AccountId) -> Result<()> {
            self.only_owner();
            self.blacklisted.take(&cleared_user);
            Ok(())
        }

        /// Destroy blacklisted user funds from total supply.
        #[ink(message)]
        fn destroy_blackfunds(&mut self, blacklisted_user: AccountId) -> Result<()> {
            self.only_owner();
            if !self.get_blacklist_status(blacklisted_user) {
                return Err(Error::NotBlacklistedUser);
            }
            let dirty_funds = self.balance_of(blacklisted_user);
            self.balances.insert(blacklisted_user, 0);
            *self.total_supply -= dirty_funds;
            self.env().emit_event(DestroyedBlackFunds {
                blacklisted_user,
                balance: dirty_funds,
            });
            Ok(())
        }
    }

    impl Erc20 {
        /// Mint a new amount of tokens
        /// these tokens are deposited into the owner address
        #[ink(message)]
        pub fn mint(&mut self, user: AccountId, amount: Balance) -> Result<()> {
            self.only_owner();
            assert_ne!(user, Default::default());
            if amount <= 0 {
                return Err(Error::InvalidAmount);
            }

            let user_balance = self.balance_of(user);
            self.balances.insert(user, user_balance + amount);
            *self.total_supply += amount;
            self.env().emit_event(Mint { user, amount });
            Ok(())
        }

        /// Burn tokens.
        /// These tokens are withdrawn from the owner address
        /// if the balance must be enough to cover the redeem
        /// or the call will fail.
        #[ink(message)]
        pub fn burn(&mut self, user: AccountId, amount: Balance) -> Result<()> {
            self.only_owner();
            if *self.total_supply < amount {
                return Err(Error::InsufficientSupply);
            }
            let user_balance = self.balance_of(user);
            if user_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(user, user_balance - amount);
            *self.total_supply -= amount;
            self.env().emit_event(Burn { user, amount });
            Ok(())
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> IResult<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(IError::InsufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner);
        }
    }
}
