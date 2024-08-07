// The most primitive representation of a Blockchain block.
pub struct Block<Header, Extrinsic> {
	// Contains metadata about the block.
	pub header: Header,
	// Represent the state transitions to be executed in this block.
	pub extrinsics: Vec<Extrinsic>,
}

// Below is an extremely simplified header containing only the current block number.
// On a real blockchain, you would expect to also find:
// - parent's block hash
// - state root
// - extrinsic root
// - etc.
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

// It's literally an external message from outside of the blockchain.
// It's a simplified version and tells s who is making the call,
// and which call they are making.
pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

// The Result type for our runtime. When the dispatch is completed successfully,
// we return `Ok(())`, otherwise return a static error message.
pub type DispatchResult = Result<(), &'static str>;

// A trait which allows us to dispatch an incoming extrinsic
// to the appropriate state transition function (STF) call.
pub trait Dispatch {
	// The type used to identify the caller of the function.
	type Caller;
	// The STF call the caller is trying to access.
	type Call;

	// A function which takes a `caller` and the `call` they want to make,
	// and returns a `Result` based on the outcome of that function call.
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
