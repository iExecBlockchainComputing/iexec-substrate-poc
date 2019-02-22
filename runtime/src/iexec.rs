/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs

use parity_codec::Encode;
use parity_codec_derive::{Encode, Decode};
use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap,ensure, dispatch::Result};
use system::ensure_signed;


use runtime_primitives::traits::Hash;


#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Task<Hash> {
    id: Hash,
    consensus: Hash
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Contribution<Hash> {
    id: Hash,
	task_id: Hash,
    contribution: Hash
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}



/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as IexecModule {
		// Just a dummy storage item. 
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(something): Option<u32>;
        Tasks get(task): map T::Hash => Task<T::Hash>;
	    ModuleSalt: u64;

		AllTasksCount get(all_tasks_count): u64;
		AllTasksArray get(task_by_index): map u64 => T::Hash;
		AllTasksIndex: map T::Hash => u64;

	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			<Something<T>>::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		pub fn create_task(origin) -> Result {
       		let sender = ensure_signed(origin)?;


            // `nonce` and `random_hash` generation can stay here
            let salt = <ModuleSalt<T>>::get();
			
            let random_hash = (
				<system::Module<T>>::random_seed(), &sender, salt)
                .using_encoded(<T as system::Trait>::Hashing::hash);

			let consensus_initial_value=0;
         	let new_task = Task {
                id: random_hash,
                consensus: consensus_initial_value.using_encoded(<T as system::Trait>::Hashing::hash),
            };
			// ACTION: Move this collision check to the `_mint()` function
            ensure!(!<Tasks<T>>::exists(random_hash), "Task already exists");


            let all_tasks_count = Self::all_tasks_count();

            let new_all_tasks_count = all_tasks_count.checked_add(1)
                .ok_or("Overflow adding a new tasks to total supply")?;

		    <Tasks<T>>::insert(random_hash, new_task);

			<AllTasksArray<T>>::insert(all_tasks_count, random_hash);
            <AllTasksCount<T>>::put(new_all_tasks_count);
            <AllTasksIndex<T>>::insert(random_hash, all_tasks_count);

			<ModuleSalt<T>>::mutate(|n| *n += 1);

			Self::deposit_event(RawEvent::TaskCreated(sender, random_hash));
			Ok(())
		}


		pub fn contribute(origin,task_id: T::Hash, contribution: T::Hash) -> Result {
			let worker = ensure_signed(origin)?;
			ensure!(<Tasks<T>>::exists(task_id), "Task must exist");
			Self::deposit_event(RawEvent::ContributionReceived(worker, task_id,contribution));		
			Ok(())
		}
		

	}
}

decl_event!(

/*
	    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance
    {
        Created(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
        Transferred(AccountId, AccountId, Hash),
        Bought(AccountId, AccountId, Hash, Balance),
    }
*/

	/// An event in this module.
	pub enum Event<T> 
		where 
		AccountId = <T as system::Trait>::AccountId,
		TaskId = <T as system::Trait>::Hash,
		Contribution = <T as system::Trait>::Hash
		{
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
		TaskCreated(AccountId,TaskId),
		ContributionReceived(AccountId,TaskId,Contribution),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<u64>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type IexecModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(IexecModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(IexecModule::something(), Some(42));
		});
	}
}
