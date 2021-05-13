#![cfg_attr(not(feature = "std"), no_std)]

///#####################################################################################################################///
///   _________  ___   ___   __  __   ___   __    ______   ______   ______    ______   ______   ___ __ __   ______      ///
/// /________/\/__/\ /__/\ /_/\/_/\ /__/\ /__/\ /_____/\ /_____/\ /_____/\  /_____/\ /_____/\ /__//_//_/\ /_____/\      ///
/// \__.::.__\/\::\ \\  \ \\:\ \:\ \\::\_\\  \ \\:::_ \ \\::::_\/_\:::_ \ \ \:::_ \ \\:::_ \ \\::\| \| \ \\::::_\/_     ///
///    \::\ \   \::\/_\ .\ \\:\ \:\ \\:. `-\  \ \\:\ \ \ \\:\/___/\\:(_) ) )_\:\ \ \ \\:\ \ \ \\:.      \ \\:\/___/\    ///
///     \::\ \   \:: ___::\ \\:\ \:\ \\:. _    \ \\:\ \ \ \\::___\/_\: __ `\ \\:\ \ \ \\:\ \ \ \\:.\-/\  \ \\::___\/_   ///
///      \::\ \   \: \ \\::\ \\:\_\:\ \\. \`-\  \ \\:\/.:| |\:\____/\\ \ `\ \ \\:\/.:| |\:\_\ \ \\. \  \  \ \\:\____/\  /// 
///       \__\/    \__\/ \::\/ \_____\/ \__\/ \__\/ \____/_/ \_____\/ \_\/ \_\/ \____/_/ \_____\/ \__\/ \__\/ \_____\/  ///
//######################################################################################################################///
                                                                                                                   
pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	// Config
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// Boiler
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Hooks
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::storage]
	#[pallet::getter(fn something)]	
	pub type Something<T> = StorageValue<_, u32>;
	
	// Calls
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;	
			<Something<T>>::put(something);			
			Self::deposit_event(Event::SomethingStored(something, who));			
			Ok(().into())
		}
	}

	#[pallet::event]
	// #[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {		
		// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}
	// Errors
	#[pallet::error]
	pub enum Error<T> {		
		NoneValue,		
		StorageOverflow,
	}
}