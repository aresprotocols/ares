#![cfg(feature = "runtime-benchmarks")] 

use super::*;
use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use frame_support::{
	traits::{Currency},
};

use sp_std::prelude::*;
use crate::Module as KittiesModule;

const SEED: u32 = 0;

// 这个方法可以返回一个有足够多钱的账号
fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
    let caller: T::AccountId = account(name, index, SEED);

    // make_free_balance_be 会确保指定的账号有指定的可用余额，如果账户不存在，也会自动创建
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

benchmarks! {
    _ {
        let b in 1 .. 1000 => ();
    }

    create_kitty {
        let b in ...;
        let caller = funded_account::<T>("caller", 0);
    }: create( RawOrigin::Signed(caller) )
    verify{

    }

    breed_kitty {
        let b in ...;
        let caller = funded_account::<T>("caller", 0);
        let _ = KittiesModule::<T>::create( RawOrigin::Signed( caller.clone() ).into() );
        let _ = KittiesModule::<T>::create( RawOrigin::Signed( caller.clone() ).into() );
    }: breed( RawOrigin::Signed(caller), 0u32.into(), 1u32.into())
    verify{

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_create_kitty::<Test>());
            assert_ok!(test_benchmark_breed_kitty::<Test>());
        });
    }
}