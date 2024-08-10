use crate::support::DispatchResult;
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

	// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(&claim)
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {
	// Create a new claim on behalf of the `caller`.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		// It will return an error if an account has already claimed that content.
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}

		self.claims.insert(claim, caller);
		Ok(())
	}

	// Revoke an existing claim on some content.
	// It should only succeed if the caller is the owner of an existing claim,
	// otherwise it will return an error.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		// Get the owner of the `claim` to be revoked.
		let _claim_owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		// Check that the `owner` matches the `caller`.
		if *_claim_owner != caller {
			return Err(&"This content is owned by another account");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		let alice = &"alice";
		let bob = &"bob";
		let first_claim = &"Hello, world!";

		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim(alice.to_string(), first_claim), Ok(()));
		assert_eq!(poe.get_claim(first_claim), Some(alice.to_string()).as_ref());
		assert_eq!(
			poe.create_claim(bob.to_string(), first_claim),
			Err("This content is already claimed.")
		);
		assert_eq!(poe.revoke_claim(alice.to_string(), first_claim), Ok(()));
		assert_eq!(poe.create_claim(bob.to_string(), first_claim), Ok(()));
	}
}
