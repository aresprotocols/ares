use crate::{Error, mock::*, Event};
use frame_support::{assert_ok, assert_noop};
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_system::{EventRecord, Phase};
use pallet_balances::RawEvent;

fn run_to_block( n: u64) {
    while System::block_number() < n {
        Kitties::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number()+1);
        System::on_initialize(System::block_number());
    }
}
//
#[test]
fn create_kitties(){
    new_test_ext().execute_with(||{
        run_to_block(10);
        //assert_eq!(Kitties::create(origin::signed(1),),Ok(()));
        Kitties::create(origin::signed(1));

        //质押的事件测试
        assert_eq!(System::events()[0].event, TestEvent::balances(RawEvent::Reserved(1, 5000)));

        //创建kitty的事件测试
        assert_eq!(System::events()[1].event, TestEvent::kitties( Event::<TestKitty>::Created( 1u64 , 0) ));
        Kitties::on_initialize(System::block_number());
    })
}

#[test]
fn create_owner_test(){
    new_test_ext().execute_with(||{
        run_to_block(10);

        let createOwner = 1;
        assert_ok!(Kitties::create(origin::signed(createOwner)));
        Kitties::create(origin::signed(1));
        let owner = Kitties::kitty_owner(0).unwrap();
        //测试owner存储单元
        assert_eq!(owner, createOwner);
        Kitties::on_initialize(System::block_number());
    })
}

#[test]
fn transfer_kitty(){
    new_test_ext().execute_with(||{
        run_to_block(10);
        assert_eq!(Kitties::create(origin::signed(1)),Ok(()));

        let owner1 = Kitties::kitty_owner(0).unwrap();
        assert_ok!(Kitties::transfer(origin::signed(1), 2, 0));
        //
        let owner2 = Kitties::kitty_owner(0).unwrap();
        //测试转移后的owner
        assert_eq!(owner2, 2);

        //let events = System::events();
        Kitties::on_initialize(System::block_number());
    })
}

#[test]
fn test_dobreed_kitty(){
    new_test_ext().execute_with(||{
        run_to_block(10);
        assert_eq!(Kitties::create(origin::signed(1)),Ok(()));
        assert_eq!(Kitties::create(origin::signed(1)),Ok(()));

        Kitties::breed(origin::signed(1), 0, 1);
        let owner2 = Kitties::kitty_owner(2).unwrap();
        assert_eq!(owner2, 1);
        //assert_noop!()
    })
}

#[test]
fn test_dobreed_kitty_same_parent(){
    new_test_ext().execute_with(||{
        run_to_block(10);
        assert_eq!(Kitties::create(origin::signed(1)),Ok(()));
        assert_eq!(Kitties::create(origin::signed(1)),Ok(()));

        assert_noop!(Kitties::breed(origin::signed(1), 0, 0), Error::<TestKitty>::RequireDifferentParent);
    })
}