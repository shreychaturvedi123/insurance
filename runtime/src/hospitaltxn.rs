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
#[derive(Encode, Decode,Clone,Eq,Debug,PartialEq)]
pub struct BillRecord<AccountId> {
	PatientKey: AccountId,
	BillId: i64,
	HospitalKey: AccountId,
	ClaimAmnt: i64,
	Prescription: u64
}

pub trait Trait: cennzx_spot::Trait {

	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Hospital {
			pub PatientKey get(patient): map T::AccountId => Option<BillRecord<T::AccountId>>;
			pub HospitalKey get(hospital): map T::AccountId => Option<BillRecord<T::AccountId>>;
			pub BillId get(bill_id): map i64 =>  Option<BillRecord<T::AccountId>>;
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
   pub fn create_bill(origin, record:BillRecord<T::AccountId>) -> Result {
  // Ensure this is from user transaction
	  let origin = ensure_signed(origin)?;
	  // Emit an on-chain event
	  <BillId<T>>::insert(record.BillId.clone(), record.clone());

	  Self::deposit_event(RawEvent::ItemCreated(record));

	  // Indicates method executed successfully
	  Ok(())
	}

	//  pub fn getBill(origin, BillId:i64) -> Result {
 //  // Ensure this is from user transaction
	//   let origin = ensure_signed(origin)?;

	//   let record = Self::bill_id(BillId.clone()).ok_or_else(||"Does not exisit")?;

	//   // Emit an on-chain event
	//   Self::deposit_event(RawEvent::ItemCreated(record));

	//   // Indicates method executed successfully
	//   Ok(())
	// }
	}
}

decl_event!(
	pub enum Event<T> where
	BillRecord = BillRecord<<T as system::Trait>::AccountId>,
	{
		/// New item created. (transactor, item_id, quantity, item, price)
		ItemCreated(BillRecord),
	}
);

