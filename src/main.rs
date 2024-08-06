mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl system::Config for Runtime {
	type AccountId = String;
	type BlockNumber = u32;
	type Nonce = u32;
}

impl Runtime {
	fn new() -> Self {
		Self { balances: balances::Pallet::new(), system: system::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	runtime.balances.set_balance(&"alice".to_string(), 100);

	// Start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// First transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(alice.clone(), bob, 30)
		.map_err(|e| eprintln!("{}", e));

	// Second transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	println!("{:#?}", runtime);
}
