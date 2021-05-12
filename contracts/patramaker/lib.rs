#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::{ vec::Vec, format };
use ink_env::Environment;

#[ink::chain_extension]
pub trait FetchPrice {
    type ErrorCode = FetchPriceErr;

    /// Note: this gives the operation a corresponding func_id (1101 in this case),
    /// and the chain-side chain_extension will get the func_id to do further operations.
    #[ink(extension = 1103, returns_result = false)]
    fn fetch_price(token_identifier: Vec<u8>) -> u64;
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FetchPriceErr {
    FailGetPrice,
}
impl ink_env::chain_extension::FromStatusCode for FetchPriceErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetPrice),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchPrice;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod patramaker {
    use dai::Erc20;
    use ink_env::call::FromAccountId;
    use ink_storage::{
        collections::HashMap as StorageMap,
        traits::{PackedLayout, SpreadLayout},
        Lazy,
    };
    use crate::{Vec, format};
    use ownership::Ownable;
    use primitive_types::U256;

    pub type CdpId = u32;
    // pub type USD = u32;

    pub const DOTS: Balance = 1_000_000_000_000;
    pub const DOT_PRICE_DECIMALS: u32 = 1_000;

    #[ink(event)]
    pub struct IssueDAI {
        #[ink(topic)]
        cdp_id: CdpId,
        #[ink(topic)]
        collateral: Balance,
        #[ink(topic)]
        dai: Balance,
    }

    #[ink(event)]
    pub struct AddCollateral {
        #[ink(topic)]
        cdp_id: CdpId,
        #[ink(topic)]
        add_collateral: Balance,
        #[ink(topic)]
        collateral_ratio: u32,
    }

    #[ink(event)]
    pub struct MinusCollateral {
        #[ink(topic)]
        cdp_id: CdpId,
        #[ink(topic)]
        minus_collateral: Balance,
        #[ink(topic)]
        collateral_ratio: u32,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        cdp_id: CdpId,
        #[ink(topic)]
        collateral: Balance,
        #[ink(topic)]
        dai: Balance,
    }

    #[ink(event)]
    pub struct Liquidate {
        #[ink(topic)]
        cdp_id: CdpId,
        #[ink(topic)]
        collateral: Balance,
        #[ink(topic)]
        keeper_reward: Balance,
    }

    #[derive(
        Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct CDP {
        pub issuer: AccountId,
        pub collateral_dot: Balance,
        // 1 DAI = 1 USD
        pub issue_dai: Balance,
        pub create_date: Timestamp,
    }

    #[ink(storage)]
    pub struct PatraMaker {
        dai_token: Erc20,
        cdps: StorageMap<CdpId, CDP>,
        cdp_count: u32,
        min_collateral_ratio: u32,
        min_liquidation_ratio: u32,
        liquidater_reward_ratio: u32,
        // dot_price: USD,
        owner: AccountId,
    }

    impl Ownable for PatraMaker {
        #[ink(constructor)]
        fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            Some(self.owner)
        }

        /// transfer contract ownership to new owner.
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) {
            assert_eq!(self.owner(), Some(self.env().caller()));
            if let Some(new_one) = new_owner {
                self.owner = new_one;
            }
        }
    }

    impl PatraMaker {
        #[ink(constructor)]
        pub fn new(dai_contract: AccountId) -> Self {
            assert_ne!(dai_contract, Default::default());
            let caller = Self::env().caller();
            let dai_token: Erc20 = FromAccountId::from_account_id(dai_contract);

            let message = format!("dai_token =  {:?}", dai_token);
            ink_env::debug_println(&message);

            Self {
                dai_token: dai_token,
                cdps: StorageMap::new(),
                cdp_count: 0,
                min_collateral_ratio: 150,
                min_liquidation_ratio: 110,
                liquidater_reward_ratio: 5,
                // dot_price: 3500,
                owner: caller,
            }
        }

        /// Adjust Min Collateral Ratio only admin
        #[ink(message)]
        pub fn adjust_mcr(&mut self, mcr: u32) {
            self.only_owner();
            self.min_collateral_ratio = mcr;
        }

        // Adjust Min Liquidation Ratio only admin
        #[ink(message)]
        pub fn adjust_mlr(&mut self, mlr: u32) {
            self.only_owner();
            self.min_liquidation_ratio = mlr;
        }

        /// Adjust Liquidater Reward Ratio only admin
        #[ink(message)]
        pub fn adjust_lrr(&mut self, lrr: u32) {
            self.only_owner();
            self.liquidater_reward_ratio = lrr;
        }

        /// Adjust dot price only admin
        // #[ink(message)]
        // pub fn adjust_dot_price(&mut self, price: USD) {
        //     self.only_owner();
        //     dot_price = price;
        // }

        /// System params
        #[ink(message)]
        pub fn system_params(&self) -> (u32, u32, u32) {
            (
                self.min_collateral_ratio,
                self.min_liquidation_ratio,
                self.liquidater_reward_ratio,
                // dot_price,
            )
        }

        /// Query cdp by id
        #[ink(message)]
        pub fn query_cdp(&self, cdp_id: CdpId) -> Option<CDP> {
            self.cdps.get(&cdp_id).cloned().and_then(|cdp| Some(cdp))
        }

        /// Stake collateral and issue dai
        #[ink(message, payable)]
        pub fn issue_dai(&mut self, cr: u32) -> (CdpId, Balance) {
            assert!(cr >= self.min_collateral_ratio);
            let caller = self.env().caller();
            let collateral = self.env().transferred_balance();
            let dot_price = self.get_dot_price();
            let token_decimals = self.dai_token.token_decimals().unwrap();
            let dai_decimals =
                10u128.saturating_pow(token_decimals as u32);
                
            let dai = collateral * dot_price as u128 * 100 * dai_decimals / DOTS / (cr * DOT_PRICE_DECIMALS) as u128;

            let message = format!("dai =  {:?}", dai);

            ink_env::debug_println(&message);

            let cdp = CDP {
                issuer: caller,
                collateral_dot: collateral,
                issue_dai: dai,
                create_date: self.env().block_timestamp(),
            };
            self.cdp_count += 1;
            self.cdps.insert(self.cdp_count, cdp);
            self.dai_token.mint(caller, dai).unwrap();

            let message = format!("dai =  {:?}", dai);
            ink_env::debug_println(&message);


            self.env().emit_event(IssueDAI {
                cdp_id: self.cdp_count,
                collateral,
                dai,
            });
            (self.cdp_count, dai)
        }

        /// Only issuer can add collateral and update collateral ratio
        #[ink(message, payable)]
        pub fn add_collateral(&mut self, cdp_id: CdpId) {
            assert!(self.cdps.contains_key(&cdp_id));
            let caller = self.env().caller();
            let collateral = self.env().transferred_balance();
            let dot_price = self.get_dot_price();
            let cdp = self.cdps.get_mut(&cdp_id).unwrap();
            assert!(cdp.issuer == caller);
            // let cr = (collateral + cdp.collateral_dot as u128) * dot_price as u128 * 100
            //     / cdp.issue_dai;
            let dai_decimals =
                10u128.saturating_pow(self.dai_token.token_decimals().unwrap() as u32);
            let cr = (collateral + cdp.collateral_dot as u128)
                * dot_price as u128
                * 100
                * dai_decimals
                / (cdp.issue_dai * DOTS * DOT_PRICE_DECIMALS as u128);

            // assert!(cr >= self.min_collateral_ratio.into());
            cdp.collateral_dot += collateral;
            self.env().emit_event(AddCollateral {
                cdp_id,
                add_collateral: collateral,
                collateral_ratio: cr as u32,
            });
        }

        /// Only issuer can minus collateral and update collateral ratio
        #[ink(message)]
        pub fn minus_collateral(&mut self, cdp_id: CdpId, collateral: Balance) {
            assert!(self.cdps.contains_key(&cdp_id));
            let caller = self.env().caller();
            let dot_price = self.get_dot_price();
            let cdp = self.cdps.get_mut(&cdp_id).unwrap();
            assert!(cdp.issuer == caller);
            // let cr =
            //     (cdp.collateral_dot - collateral) * dot_price as u128 * 100 / cdp.issue_dai;
            let dai_decimals =
                10u128.saturating_pow(self.dai_token.token_decimals().unwrap() as u32);
            let cr =
                (cdp.collateral_dot - collateral) * dot_price as u128 * 100 * dai_decimals
                    / (cdp.issue_dai * DOTS * DOT_PRICE_DECIMALS as u128);

            assert!(cr >= self.min_collateral_ratio.into());
            cdp.collateral_dot -= collateral;
            self.env().transfer(caller, collateral).unwrap();
            self.env().emit_event(MinusCollateral {
                cdp_id,
                minus_collateral: collateral,
                collateral_ratio: cr as u32,
            });
        }

        /// Only issuer can withdraw
        #[ink(message)]
        pub fn withdraw_dot(&mut self, cdp_id: CdpId, dai: Balance) -> Balance {
            assert!(self.cdps.contains_key(&cdp_id));
            let caller = self.env().caller();
            let cdp = self.cdps.get_mut(&cdp_id).unwrap();
            assert!(cdp.issuer == caller);
            // let cr = (cdp.collateral_dot * dot_price as u128 * 100 / cdp.issue_dai) as u32;
            // assert!(cr >= self.min_collateral_ratio);
            assert!(dai <= cdp.issue_dai);

            let bt: U256 = cdp.collateral_dot.into();
            let bi: U256 = dai.into();
            let ui: U256 = cdp.issue_dai.into();
            let r = bt * bi / ui;
            let dot = r.as_u128();

            cdp.collateral_dot -= dot;
            cdp.issue_dai -= dai;
            self.env().transfer(caller, dot).unwrap();
            self.dai_token.burn(caller, dai).unwrap();
            self.env().emit_event(Withdraw {
                cdp_id,
                collateral: dot,
                dai,
            });
            dot
        }

        /// Anyone can invoke collateral liquidation if current collateral ratio lower than minimum
        #[ink(message)]
        pub fn liquidate_collateral(&mut self, cdp_id: CdpId, dai: Balance) {
            assert!(self.cdps.contains_key(&cdp_id));
            let dot_price = self.get_dot_price();
            let cdp = self.cdps.get_mut(&cdp_id).unwrap();
            // let cr = (cdp.collateral_dot * dot_price as u128 * 100 / cdp.issue_dai) as u32;
            let dai_decimals =
                10u128.saturating_pow(self.dai_token.token_decimals().unwrap() as u32);
            let cr = (cdp.collateral_dot * dot_price as u128 * 100 * dai_decimals
                / (cdp.issue_dai * DOTS * DOT_PRICE_DECIMALS as u128)) as u32;
            assert!(cr <= self.min_liquidation_ratio);
            let owner = cdp.issuer;
            let dot =
                dai * DOTS * DOT_PRICE_DECIMALS as u128 / (dot_price as u128 * dai_decimals);
            cdp.issue_dai = cdp.issue_dai.saturating_sub(dai);
            // let keeper_reward =
            //     dai * DOTS * self.liquidater_reward_ratio as u128 * DOT_PRICE_DECIMALS as u128
            //         / (100 * dot_price as u128 * dai_decimals);
            let keeper_reward = dai * DOTS * self.liquidater_reward_ratio as u128
                / (dot_price as u128 * dai_decimals);

            assert!(dot + keeper_reward <= cdp.collateral_dot);
            cdp.collateral_dot = cdp.collateral_dot - dot - keeper_reward;

            let mut rest_dot = 0_u128;
            if cdp.issue_dai == 0 && cdp.collateral_dot > 0 {
                rest_dot = cdp.collateral_dot;
                cdp.collateral_dot = 0;
            }
            let caller = self.env().caller();
            assert!(self.env().transfer(caller, dot + keeper_reward).is_ok());
            assert!(self.dai_token.burn(caller, dai).is_ok());
            if rest_dot > 0 {
                assert!(self.env().transfer(owner, rest_dot).is_ok());
            }
            self.env().emit_event(Liquidate {
                cdp_id,
                collateral: dot,
                keeper_reward,
            });
        }

        /// Returns the total issuers、total collateral、total issue dai.
        #[ink(message)]
        pub fn total_supply(&self) -> (u32, Balance, Balance) {
            let mut issuers = Vec::new();
            let total_collateral: Balance = self.env().balance();
            let total_issue_dai: Balance = self.dai_token.total_supply();
            for (_k, v) in self.cdps.iter() {
                if !issuers.contains(&v.issuer) {
                    issuers.push(v.issuer);
                }
                // total_collateral += v.collateral_dot;
                // total_issue_dai += v.issue_dai;
            }
            (issuers.len() as u32, total_collateral, total_issue_dai)
        }
        #[ink(message)]
        pub fn dot_price(&self) -> u64 {

            self.get_dot_price()
        }

        /// Returns the total cdp amount.
        #[ink(message)]
        pub fn cdp_count(&self) -> u32 {
            self.cdp_count
        }

        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner);
        }

        // Get the DOT price
        fn get_dot_price(&self) -> u64{
            let price: u64 = self.env().extension().fetch_price("dotusdt".as_bytes().to_vec()).unwrap();
            price
        }
    }
}