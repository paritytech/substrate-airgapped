use super::{system::System, Parameter};
use crate::{runtimes::Runtime, util::int_as_human};
use codec::{Decode, Encode};
use core::fmt::{self, Debug, Display};
use sp_core::crypto::Ss58Codec;
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
		+ MaybeSerialize
		+ Display; // This is not included in the substrate runtime, but helps us impl pretty print
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

impl<T> super::PalletCall for Transfer<T>
where
	T: Balances + System,
{
	const CALL: &'static str = "transfer";
	const PALLET: &'static str = "Balances";
}

impl<T> Display for Transfer<T>
where
	T: Balances + System + Runtime,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"balances::Transfer\n{sp}to: {}\n{sp}amount: {}",
			self.to.to_ss58check_with_version(<T as Runtime>::SS58_ADDRESS_FORMAT),
			int_as_human(self.amount),
			sp = format!("{:indent$}", "", indent = 8)
		)
	}
}
