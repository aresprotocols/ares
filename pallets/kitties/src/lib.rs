#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module,decl_storage, decl_event, decl_error, StorageValue, ensure, StorageMap, traits::Randomness, Parameter,traits::{ExistenceRequirement ,Get, Currency, ReservableCurrency}
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError,traits::{AtLeast32Bit,Bounded}};

mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// 定义一个 kitty 的数据结构
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: frame_system::Config {
    // 如果有触发事件，就必须包含这一行
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    type Randomness: Randomness<Self::Hash>;

    // 定义 KittyIndex 类型，要求实现指定的 trait
    // Parameter 表示可以用于函数参数传递
    // AtLeast32Bit 表示转换为 u32 不会造成数据丢失
    // Bounded 表示包含上界和下界
    // Default 表示有默认值
    // Copy 表示可以实现 Copy 方法
    type KittyIndex: Parameter + AtLeast32Bit + Bounded + Default + Copy;
    // 创建 Kitty 的时候，需要质押的代币
    type NewKittyReserve: Get<BalanceOf<Self>>;
    // Currency 类型，用于质押等于资产相关的操作
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

// 定义数据存储
decl_storage! {
	// 定义所储存的数据是属于 KittiesModule 的，( 这个需要和 runtime > lib.rss > construct_runtime 部分引用这个 pallet 的名称对应？）
	// T: Config 里边的 Config 就是第17行定义的 Config
	trait Store for Module<T: Config> as KittiesModule {
		// 保存所有 kitty 的数据，用 KittyIndex 作为健值
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
		// 保存 kitty 的总数，严格上来说，应该是最大的 Kitty 的健值索引，因为如果支持 kitty 的删除，实现上就不对了。
		// T::AccountId 就是指第 17 行定义的 trait 的 AccountId 类型，而这边定义的 AccountId 是继承自 frame_system::Config 里边的 AccountId
		pub KittiesCount get(fn kitties_count): T::KittyIndex;
		// 保存每一只猫归那个拥有者
		pub KittyOwners get(fn kitty_owners): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;
		// 记录某个拥有者与猫之间的关系
		pub OwnedKitties get(fn owned_kitties):double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::KittyIndex => Option<T::KittyIndex>;
		// 记录某只猫的父母，因为猫可能没有父母，所以用 Option
		pub KittyParents get(fn kitty_parents):map hasher(blake2_128_concat) T::KittyIndex => Option<(T::KittyIndex, T::KittyIndex)>;
		// 记录某只猫的孩子们，第一个值是主猫，第二个是孩子，值也是孩子
		pub KittyChildren get(fn kitty_children):double_map hasher(blake2_128_concat) T::KittyIndex, hasher(blake2_128_concat) T::KittyIndex => Option<T::KittyIndex>;
		// 记录某只猫的伴侣，第一个是主猫，第二个是伴侣猫，值是伴侣猫
		pub KittyPartners get(fn kitty_partners):double_map hasher(blake2_128_concat) T::KittyIndex, hasher(blake2_128_concat) T::KittyIndex => Option<T::KittyIndex>;

		pub KittyPrices get(fn kitty_prices): map hasher(blake2_128_concat) T::KittyIndex => Option<BalanceOf<T>>;
	}
}

// 定义事件
decl_event!(
	// where 后边的部分，是表示在 Event 里边需要用的一些类型来自哪个 Config 定义
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, KittyIndex = <T as Config>::KittyIndex, BalanceOf = BalanceOf<T> {
		Created(AccountId, KittyIndex),
		Transferred(AccountId, AccountId, KittyIndex),
		KittyAsk(AccountId, KittyIndex, Option<BalanceOf>),
	}
);

// 定义错误信息
decl_error! {
	pub enum Error for Module<T: Config> {
		KittiesCountOverflow,
		KittyNotExists,
		NotKittyOwner,
		TransferToSelf,
		RequiredDiffrentParent,
		MoneyNotEnough,
		UnReserveMoneyNotEnough,
		AlreadyOwned,
		NotForSale,
		PriceTooLow,
	}
}

// 定义可被调用的方法
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// 如果有触发错误信息，必须包含这一行
		type Error = Error<T>;
		// 如果有触发事件，必须包含这一行
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create(origin){
			// 加 “?” 只提取正确时候返回的数据
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;
			let dna = Self::random_value(&sender);
			let kitty = Kitty(dna);

			// 质押指定数量的资产，如果资产质押失败，会报错【质押会触发时间，做测试的时候需要注意】
			T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough )?;

			Self::insert_kitty(&sender, kitty_id, kitty, None);
			Self::deposit_event(RawEvent::Created(sender, kitty_id));
		}
		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex){
			let sender = ensure_signed(origin)?;
			// 判断 KittyIndex 是否存在，通过 ok_or 将错误抛出来，如果没有将返回一个 option 类型的数据
			let owner = Self::kitty_owners(kitty_id).ok_or( Error::<T>::KittyNotExists )?;
			// 判断 KittyIndex 是否属于发送者
			ensure!(owner == sender, Error::<T>::NotKittyOwner);

			// 不能转让给自己
			ensure!(to != sender, Error::<T>::TransferToSelf);

			// 质押被转让人的代币
			T::Currency::reserve(&to, T::NewKittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough )?;

			// 解质押转出人的代币
			// 如果配置的质押代币数量变化了，可能这里会出问题。其实最好的方式是每个 kitty 都记录下，它当时质押的代币数量
			T::Currency::unreserve(&sender, T::NewKittyReserve::get());

			// 修改 KITTY 的拥有人
			KittyOwners::<T>::insert(kitty_id, &to);
			// 从之前的拥有人中删除关系
			OwnedKitties::<T>::remove(&sender, kitty_id);
			OwnedKitties::<T>::insert(&to, kitty_id, kitty_id);

			// 触发转让的事件
			Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
		}
		#[weight = 0]
		pub fn breed(origin, kitty_id1: T::KittyIndex, kitty_id2: T::KittyIndex){
			let sender = ensure_signed(origin)?;
			let new_kitty_id = Self::do_breed(&sender, kitty_id1, kitty_id2)?;

			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
		}
		#[weight = 0]
		pub fn ask(origin, kitty_id: T::KittyIndex, new_price: Option<BalanceOf<T>>){
			let sender = ensure_signed(origin)?;
			// 判定是不是 kitty 的主人
			ensure!( Some( sender.clone() ) == Self::kitty_owners(kitty_id), Error::<T>::NotKittyOwner);
			
			// mutate_exists ：修改 map 指定键的值，如果为 none 就删除，第二个参数是一个闭包，提供的参数是键值 
			<KittyPrices<T>>::mutate_exists(kitty_id, |price| *price = new_price);

			// 触发一个挂单的事件
			Self::deposit_event(RawEvent::KittyAsk(sender, kitty_id, new_price));
		}
		#[weight = 0]
		pub fn buy(origin, kitty_id: T::KittyIndex, price: BalanceOf<T>){
			let sender = ensure_signed(origin)?;
			// 检查是否存在，顺便提取出售者
			let owner = Self::kitty_owners(kitty_id).ok_or( Error::<T>::KittyNotExists )?;
			// 已经是自己的不再折腾
			ensure!( sender.clone() != owner, Error::<T>::AlreadyOwned);
			let kitty_price = Self::kitty_prices(kitty_id).ok_or( Error::<T>::NotForSale)?;
			// 确认出价是不是太低
			ensure!( kitty_price <= price, Error::<T>::PriceTooLow);

			// 转质押 + 扣款
			// 对于购买者，先质押购买的和创建抵押的
			T::Currency::reserve(&sender, T::NewKittyReserve::get() + kitty_price ).map_err(|_| Error::<T>::MoneyNotEnough )?;
			// 释放卖出者之前质押的
			T::Currency::unreserve(&owner, T::NewKittyReserve::get());
			// 释放购买者需要支付用来质押的
			T::Currency::unreserve(&sender, kitty_price);
			// 转账
			T::Currency::transfer(&sender, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

			// 移除价格挂单
			<KittyPrices::<T>>::remove(&kitty_id);
			// 转移 Kitty
			<KittyOwners::<T>>::insert(&kitty_id, sender.clone() );

			// 触发所有权转让的事件
			Self::deposit_event(RawEvent::Transferred(owner, sender, kitty_id));
		}
	}
}

impl<T: Config> Module<T> {
    // 获取下一个
    fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError>{
        let kitty_id = Self::kitties_count();
        if kitty_id == T::KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }
        Ok(kitty_id)
    }
    fn random_value(sender : &T::AccountId) -> [u8; 16] {
        let payload = (
            T::Randomness::random_seed(),	// 通过最近区块信息生成的随机数种子
            &sender,
            <frame_system::Module<T>>::extrinsic_index() // 当前交易在区块中的顺序
        );
        payload.using_encoded(blake2_128)
    }
    // 插入一个 kitty ，因为父母可能不存在，所以parent 需要用 Option
    fn insert_kitty(owner : &T::AccountId, kitty_id : T::KittyIndex, kitty : Kitty, parent: Option<(T::KittyIndex, T::KittyIndex)> ){
        // 保存 Kitty
        <Kitties::<T>>::insert(kitty_id, kitty);
        // 更新 Kitty 数量，当前 ID+1
        <KittiesCount::<T>>::put(kitty_id+1u32.into());
        // 保存 Kitty 的所有关系
        <KittyOwners::<T>>::insert(kitty_id, owner);
        // 保存拥有者拥有的 Kitty 数据
        <OwnedKitties::<T>>::insert(owner, kitty_id, kitty_id);
        // 保存 Kitty 的父母相关的数据，因为无父母的情况，就不管了
        match parent {
            Some((parent_id1, parent_id2)) =>{
                // 保存 kitty 的父母
                <KittyParents::<T>>::insert(kitty_id, (parent_id1, parent_id2) );
                // 保存父母的孩子
                <KittyChildren::<T>>::insert(parent_id1, kitty_id, kitty_id);
                <KittyChildren::<T>>::insert(parent_id2, kitty_id, kitty_id);
                // 保存父母的伴侣关系
                <KittyPartners::<T>>::insert(parent_id1, parent_id2, parent_id2);
                <KittyPartners::<T>>::insert(parent_id2, parent_id1, parent_id1);
            }
            _ => (),
        }
    }

    fn do_breed(owner : &T::AccountId, kitty_id1: T::KittyIndex, kitty_id2: T::KittyIndex) -> sp_std::result::Result<T::KittyIndex, DispatchError>{
        // 不允许相同的猫进行繁殖
        ensure!( kitty_id1 != kitty_id2, Error::<T>::RequiredDiffrentParent);

        // 判断 KittyIndex 是否存在，通过 ok_or 将错误抛出来，如果没有将返回一个 option 类型的数据
        let owner1 = Self::kitty_owners(kitty_id1).ok_or( Error::<T>::KittyNotExists )?;
        let owner2 = Self::kitty_owners(kitty_id2).ok_or( Error::<T>::KittyNotExists )?;
        // 判断 KittyIndex 是否属于发送者
        ensure!(owner1 == *owner, Error::<T>::NotKittyOwner);
        ensure!(owner2 == *owner, Error::<T>::NotKittyOwner);

        let kitty_1 = Self::kitties(kitty_id1).ok_or( Error::<T>::KittyNotExists )?;
        let kitty_2 = Self::kitties(kitty_id2).ok_or( Error::<T>::KittyNotExists )?;

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty_1.0;
        let kitty2_dna = kitty_2.0;
        let selector = Self::random_value(&owner);

        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        let kitty = Kitty(new_dna);

        T::Currency::reserve(&owner, T::NewKittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough )?;

        Self::insert_kitty(owner, kitty_id, kitty, Some((kitty_id1, kitty_id2)));

        Ok(kitty_id)
    }
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8{
    (selector & dna1 ) | (!selector & dna2)
}