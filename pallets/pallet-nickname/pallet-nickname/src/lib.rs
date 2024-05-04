#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{Currency, ReservableCurrency},
	};
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn nickname_of)]
	pub type Nicknames<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Vec<u8>, ConstU32<3>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NicknameSet(T::AccountId, Vec<u8>),
		NicknameUpdated(T::AccountId, Vec<u8>),
		NicknameCleared(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyNicknames,
		NicknameInUse,
		NicknameNotFound,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 6. 可调用函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn set_nickname(origin: OriginFor<T>, nickname: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(nickname.len() <= 32, Error::<T>::TooManyNicknames);
			ensure!(!Nicknames::<T>::contains_key(&nickname), Error::<T>::NicknameInUse);

			Nicknames::<T>::try_mutate(&sender, |names| {
				names.try_push(nickname.clone()).map_err(|_| Error::<T>::TooManyNicknames)?;
				Ok(())
			})?;

			Self::deposit_event(Event::NicknameSet(sender, nickname));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn clear_nickname(origin: OriginFor<T>, nickname: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Nicknames::<T>::contains_key(&sender), Error::<T>::NicknameNotFound);

			Nicknames::<T>::mutate(&sender, |names| {
				let index = names.iter().position(|n| *n == nickname).ok_or(Error::<T>::NicknameNotFound)?;
				names.remove(index);
			});

			Self::deposit_event(Event::NicknameCleared(sender));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn update_nickname(origin: OriginFor<T>, old_nickname: Vec<u8>, new_nickname: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(nickname.len() <= 32, Error::<T>::TooManyNicknames);
			ensure!(!Nicknames::<T>::contains_key(&new_nickname), Error::<T>::NicknameInUse);
			ensure!(Nicknames::<T>::contains_key(&sender), Error::<T>::NicknameNotFound);

			Nicknames::<T>::mutate(&sender, |names| {
				let index = names.iter().position(|n| *n == old_nickname).ok_or(Error::<T>::NicknameNotFound)?;
				names[index] = new_nickname.clone();
			});

			Self::deposit_event(Event::NicknameUpdated(sender, new_nickname));
			Ok(())
		}
	}
}