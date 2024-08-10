mod balances;
mod proof_of_existence;
mod support;
mod system;

// Need to import this to access the `dispatch` fn
use crate::support::Dispatch;

// Concrete types useful in our simple state machine.
// Modules are configured for these types directly,
// and they satisfy all of our trait requirements.
mod types {
	use crate::RuntimeCall;

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	poe: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
			RuntimeCall::ProofOfExistence(call) => {
				self.poe.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
}

impl Runtime {
	fn new() -> Self {
		Self {
			balances: balances::Pallet::new(),
			system: system::Pallet::new(),
			poe: proof_of_existence::Pallet::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		// Increment the system's block number.
		self.system.inc_block_number();
		// Check if the block number of the incoming block matches the current block
		// number, if not return an error.
		if self.system.block_number() != block.header.block_number {
			return Err(&"Block number does not match what is expected");
		}

		// Iterate over the extrinsics in the block
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			// Increment the nonce of the caller.
			self.system.inc_nonce(&caller);
			// Dispatch the extrinsic using the `caller` and the `call`
			// contained in the extrinsic.
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
			// Handle errors from `dispatch` same as we did for individual
			// calls: printing any error and capturing the result.
		}
		Ok(())
	}
}

fn main() {
	// Create a new instance of the Runtime,
	// with all the modules it uses.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&"alice".to_string(), 100);

	// Create a block and an extrinsic
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 30,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::transfer { to: charlie, amount: 50 }),
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
					claim: &"Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
					claim: &"Hello, world!",
				}),
			},
		],
	};

	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
					claim: &"Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
					claim: &"Hello, world!",
				}),
			},
		],
	};

	// execute blocks, otherwise panic with "invalid block"
	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");

	println!("{:#?}", runtime);
}
