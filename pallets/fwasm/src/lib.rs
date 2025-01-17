#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, TypeInfo, Debug)]
    pub struct Sugar<AccountId, Hash> {
        pub name: Vec<u8>,
        pub account: AccountId,
        pub wasm_url: Vec<u8>,
        pub wasm_hash: Hash,
        pub wasm_version: u32,
        pub energy: u128,
        pub current_event: u64,
        pub root_state: Hash,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::unbounded]
    pub type Cofhe<T: Config> = StorageMap<
        Hasher = Blake2_128Concat,
        Key = T::Hash,
        Value = Sugar<T::AccountId, T::Hash>,
        QueryKind = OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SugarCreated {
            name: Vec<u8>,
            account: T::AccountId,
            wasm_url: Vec<u8>,
            wasm_hash: T::Hash,
            wasm_version: u32,
            energy: u128,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn create_fwasm(
            origin: OriginFor<T>,
            name: Vec<u8>,
            wasm_url: Vec<u8>,
            wasm_hash: T::Hash,
            energy: Option<u128>,
        ) -> DispatchResult {
            let author = ensure_signed(origin)?;
            ensure!(name.len() <= 80, "Name too long");
            ensure!(wasm_url.len() <= 256, "Wasm URL too long");
            let hash = T::Hashing::hash(&(author.clone(), name.clone()).encode());
            ensure!(!Cofhe::<T>::contains_key(&hash), "Cofhe already exists");
            Cofhe::<T>::insert(
                &hash,
                Sugar {
                    name: name.clone(),
                    account: author.clone(),
                    wasm_url: wasm_url.clone(),
                    wasm_hash,
                    wasm_version: 0,
                    energy: energy.unwrap_or_default(),
                    current_event: 0,
                    root_state: T::Hash::default(),
                },
            );
            Self::deposit_event(Event::SugarCreated {
                name,
                account: author,
                wasm_url,
                wasm_hash,
                wasm_version: 0,
                energy: energy.unwrap_or_default(),
            });
            Ok(())
        }
    }
}
