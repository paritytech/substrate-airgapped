use super::{system::System, Parameter};
use codec::{Decode, Encode};
use core::fmt::Debug;
use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member};

/// The subset of the `pallet_balances::Trait` that a Runtime can implement.
pub trait Balances: System {
	/// The balance of an account.
	type Balance: Parameter
		+ codec::Codec
		+ Default
		+ Copy
		+ Debug
		+ From<<Self as System>::BlockNumber>
		+ Member
		+ AtLeast32Bit
		+ MaybeSerialize;
}

/// Transfer some liquid free balance to another account.
///
/// `transfer` will set the `FreeBalance` of the sender and receiver.
/// It will decrease the total issuance of the system by the `TransferFee`.
/// If the sender's account is below the existential deposit as a result
/// of the transfer, the account will be reaped.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct Transfer<T: Balances + System> {
	/// Destination of the transfer.
	pub to: <T as System>::Address,
	/// Amount to transfer.
	#[codec(compact)]
	pub amount: T::Balance,
}

impl<T> super::ModuleCall for Transfer<T>
where
	T: Balances + System,
{
	fn call(&self) -> &'static str {
		"transfer"
	}
	fn pallet(&self) -> &'static str {
		"Balances"
	}
}
