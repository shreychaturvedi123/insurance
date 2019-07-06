/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, Parameter, ensure};
use runtime_primitives::traits::{CheckedAdd, CheckedMul, As};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use crate::hospitaltxn;
use crate::hospitaltxn::{PatientKey,HospitalKey};

#[derive(Encode, Decode,Clone,Eq,Debug,PartialEq)]

pub struct HospitalRecord<AccountId> {
	HospitalKey: AccountId,
	Code: i64,
}

pub trait Trait: hospitaltxn::Trait {

	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Insurer {
			pub HospitalKey get(hospital): map T::AccountId => Option<HospitalRecord<T::AccountId>>;
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
   pub fn add_hospital(origin, record:HospitalRecord<T::AccountId>) -> Result {
  // Ensure this is from user transaction
	  let origin = ensure_signed(origin)?;
	  // Emit an on-chain event
	  <HospitalKey<T>>::insert(record.HospitalKey.clone(), record.clone());

	  Self::deposit_event(RawEvent::ItemCreated(record));

	  // Indicates method executed successfully
	  Ok(())
	}
	}
}

decl_event!(
	pub enum Event<T> where
	HospitalRecord = HospitalRecord<<T as system::Trait>::AccountId>,
	{
		/// New item created. (transactor, item_id, quantity, item, price)
		ItemCreated(HospitalRecord),
	}
);

