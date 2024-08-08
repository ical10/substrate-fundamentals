use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	// The type which represents the content that can be claimed using this pallet.
	// The content can be in the form of bytes, or the hash for more economical alternative.
	// This flexibility could help the runtime developer.
	type Content: Debug + Ord;
}

// The Proof of Existence Module: a simple moudle that allows accounts
// to claim existence over some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A simple storage map from content to the owner of that content.
	// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}
}
