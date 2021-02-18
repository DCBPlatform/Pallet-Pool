#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, 
	decl_event, 
	decl_module, 
	decl_storage, 
	//ensure, 
	//dispatch::DispatchResult,
	traits::{
		Currency, 
		//Get,
		ReservableCurrency, 
		//ExistenceRequirement::AllowDeath
	},
};
use frame_system::{
	self as system, 
	//ensure_signed,
	//ensure_root
};
use parity_scale_codec::{
	Decode, 
	Encode
};
// use sp_runtime::{
// 	traits::{AccountIdConversion, Saturating, Zero},
// 	ModuleId,
// };
use sp_std::prelude::*;
//use pallet_token as Token;



#[cfg(test)]
mod tests;

//const PALLET_ID: ModuleId = ModuleId(*b"pools888");


pub trait Trait: system::Trait  + pallet_token::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: ReservableCurrency<Self::AccountId>;
}

pub type PoolIndex = u128;
type AccountIdOf<T> = <T as system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;
type PoolInfoOf<T> = PoolInfo<AccountIdOf<T>, <T as system::Trait>::BlockNumber>;

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PoolInfo<AccountId, BlockNumber> {
	pool_type: u32, // 0 means capital pool, 1 means liquidity pool
	banker: AccountId,
	created: BlockNumber
}


decl_storage! {
	trait Store for Module<T: Trait> as Store {

		Pools get(fn pools): map hasher(blake2_128_concat) PoolIndex => Option<PoolInfoOf<T>>;
		PoolCount get(fn pool_count): PoolIndex;

	}
}

decl_event! {
	pub enum Event<T> where
		Balance = BalanceOf<T>,
		AccountId = <T as system::Trait>::AccountId,
		<T as system::Trait>::BlockNumber,
	{
		/// Test event \[amount, account, block_number\]
		TestEvent(Balance, AccountId, BlockNumber),	
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// ErrorUnkwown
		ErrorUnkwown,	
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		type Error = Error<T>;		
		
		#[weight = 10_000]
		fn exchange_accounts(origin, account_type:u32, account_id:AccountIdOf<T>) {}

		// #[weight = 10_000]
		// fn create(
		// 	origin
		// ) {
		// 	let creator = ensure_signed(origin)?;
		// 	let now = <system::Module<T>>::block_number();
		// 	//ensure!(end > now, Error::<T>::EndTooEarly);
		// 	let index = PoolCount::get();
		// 	PoolCount::put(index + 1);

		// 	let imb = T::Currency::withdraw(
		// 		&creator,
		// 		deposit,
		// 		WithdrawReasons::from(WithdrawReason::Transfer),
		// 		ExistenceRequirement::AllowDeath,
		// 	)?;			
		// 	T::Currency::resolve_creating(&Self::pool_account_id(index), imb);

		// 	<Pools<T>>::insert(index, PoolInfo {
		// 		banker: creator,
		// 		created: now
		// 	});

		// }	
		
		// #[weight = 10_000]
		// fn deposit(
		// 	origin
		// ) {
		// 	let creator = ensure_signed(origin)?;
		// 	let now = <system::Module<T>>::block_number();
		// }	
		
		// #[weight = 10_000]
		// fn withdraw(
		// 	origin
		// ) {
		// 	let creator = ensure_signed(origin)?;
		// 	let now = <system::Module<T>>::block_number();
		// }			
							
		fn on_finalize(now: T::BlockNumber) {
			
			let _now = now;
			//Self::play();

		}

	}
}

impl<T: Trait> Module<T> {
	
	// fn play() -> () {
	// 	let total_supply = <Token::Module<T>>::tengok_ni(0);
	// }

	// pub fn pool_account_id(index: PoolIndex) -> T::AccountId {
	// 	PALLET_ID.into_sub_account(index)
	// }

	// pub fn id_from_index(index: PoolIndex) -> child::ChildInfo {
	// 	let mut buf = Vec::new();
	// 	buf.extend_from_slice(b"alibaba8");
	// 	buf.extend_from_slice(&index.to_le_bytes()[..]);

	// 	child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
	// }	

	// pub fn pool_deposit(index: PoolIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
	// 	let id = Self::id_from_index(index);
	// }

	// pub fn pool_withdraw(index: PoolIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
	// 	let id = Self::id_from_index(index);
	// }	
	
}

