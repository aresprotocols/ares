use super::*;
use codec::Encode;
use frame_support::sp_runtime::app_crypto::Public;
use frame_support::sp_runtime::generic::{Era, SignedPayload};
use frame_support::sp_runtime::traits;
use frame_support::sp_runtime::traits::AccountIdConversion;
use frame_support::traits::FindAuthor;
use frame_support::ConsensusEngineId;
use ares_oracle;
use ares_oracle::Config;
use sp_runtime::traits::StaticLookup;
use sp_runtime::{MultiAddress, SaturatedConversion};
use crate::governance::part_technical::TechnicalCollective;
pub use ares_oracle::LOCAL_STORAGE_PRICE_REQUEST_DOMAIN;


// An index to a block.
pub type BlockNumber = u32;

pub type EnsureRootOrHalfTechnicalCollective = EnsureOneOf<
	AccountId,
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, TechnicalCollective>,
>;

parameter_types! {
	pub const UnsignedPriority: u64 = 1 << 20;
	// pub const NeedVerifierCheck: bool = true;
	pub const FractionLengthNum: u32 = 2;
	pub const CalculationKind: u8 = 1;
}

impl ares_oracle::aura_handler::Config for Runtime {

}

impl ares_oracle::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type OffchainAppCrypto = ares_oracle::crypto::OcwAuthId<Self>;
	// type AuthorityAres = ares_oracle::crypto::AuthorityId;
	type AuthorityAres = AuraId;
	// type CheckDeposit = AresChallenge;
	// type UnsignedInterval = UnsignedInterval;
	type UnsignedPriority = UnsignedPriority;
	// type FindAuthor = staking_extend::OcwFindAuthor<Babe, Self> ; // OcwFindAuthor<Babe>;// Babe;
	// type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self,Babe>;
	type FindAuthor =
		OcwFindAccountFromAuthorIndex<Self, pallet_aura::FindAccountFromAuthorIndex<Self, Aura>>;
	// type FindAuthor = OcwFindAccountFromAuthorIndex<Self, Aura>;
	type FractionLengthNum = FractionLengthNum;
	type CalculationKind = CalculationKind;
	// type RequestOrigin = pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, TechnicalCollective> ; // frame_system::EnsureRoot<AccountId>;
	type RequestOrigin = EnsureRootOrHalfTechnicalCollective ;
	// type RequestOrigin = frame_system::EnsureRoot<AccountId>;
	type ValidatorAuthority = <Self as frame_system::Config>::AccountId;
	// type VMember = StakingExtend;
	type VMember = MemberExtend;
	type AuthorityCount = ares_oracle::aura_handler::Pallet<Runtime>;
	type OcwFinanceHandler = OcwFinance;
	type AresIStakingNpos = Self;
}



/// Wraps the author-scraping logic for consensus engines that can recover
/// the canonical index of an author. This then transforms it into the
/// registering account-ID of that session key index.
pub struct OcwFindAccountFromAuthorIndex<T, Inner>(sp_std::marker::PhantomData<(T, Inner)>);

impl<T: Config, Inner: FindAuthor<AuraId>> FindAuthor<T::AccountId>
	for OcwFindAccountFromAuthorIndex<T, Inner>
where
	sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>,
	u64: From<<T as frame_system::Config>::BlockNumber>,
	<T as frame_system::Config>::AccountId: From<sp_runtime::AccountId32>,
{
	fn find_author<'a, I>(digests: I) -> Option<T::AccountId>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		let find_auraid = Inner::find_author(digests)?;

		let mut a = [0u8; 32];
		a[..].copy_from_slice(&find_auraid.to_raw_vec());
		// extract AccountId32 from store keys
		let owner_account_id32 = sp_runtime::AccountId32::new(a);
		let authro_account_id = owner_account_id32.clone().into();
		Some(authro_account_id)
	}
}

// pub struct OcwFindAuthor<Inner>(sp_std::marker::PhantomData<Inner>);

// const TEST_ID: ConsensusEngineId = [1, 2, 3, 4];
// pub struct OcwFindAuthor<Inner>(sp_std::marker::PhantomData<Inner>);
// impl <Inner: FindAuthor<u32>> FindAuthor<u32> for OcwFindAuthor<Inner> {
//     fn find_author<'a, I>(digests: I) -> Option<u32> where
//         I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
//     {
//         log::info!("RUN OcwFindAuthor<Inner> = Bebing.");
//         let author_index =  Inner::find_author(digests);
//         log::info!("RUN OcwFindAuthor<Inner> = Value = {:?} ", author_index);
//         author_index
//         // for (id, data) in digests {
//         //     if id == TEST_ID {
//         //         return AccountId::decode(&mut &data[..]).ok();
//         //     }
//         // }
//         // None
//     }
// }

// pub struct FindAccountFromAuthorIndex<T, Inner>(sp_std::marker::PhantomData<(T, Inner)>);
// impl<T: Config, Inner: FindAuthor<u32>> FindAuthor<T::ValidatorId>
// for FindAccountFromAuthorIndex<T, Inner>
// {
//     // fn find_author<'a, I>(digests: I) -> Option<T::ValidatorId>
//     //     where I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
//     // {
//     //     let i = Inner::find_author(digests)?;
//     //
//     //     let validators = <Module<T>>::validators();
//     //     validators.get(i as usize).map(|k| k.clone())
//     // }
//
//     fn find_author<'a, I>(digests: I) -> Option<AccountId> where
//         I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
//     {
//         for (id, data) in digests {
//             if id == TEST_ID {
//                 return AccountId::decode(&mut &data[..]).ok();
//             }
//         }
//         None
//     }
// }

//
impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	//
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as traits::Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(Call, <UncheckedExtrinsic as traits::Extrinsic>::SignaturePayload)> {
		let tip = 0;
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let era = Era::mortal(period, current_block);
		let extra = (
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(era),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);

		// TODO::Sign one of your own data, the signed data is called raw_payload
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		// let address = Indices::unlookup(account);
		let address = MultiAddress::Id(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature.into(), extra)))
	}
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = Call;
}
