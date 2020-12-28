use crate::frame::{system::System, Parameter};
use core::fmt::Debug;
use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member};
use codec::{Encode, Decode};
/// The subset of the `pallet_balances::Trait` that a Runtime can implement.
pub trait Balances: System {
	/// The balance of an account.
	type Balance: Parameter
		+ Member
		+ AtLeast32Bit
		+ codec::Codec
		+ Default
		+ Copy
		+ MaybeSerialize
		+ Debug
		+ From<<Self as System>::BlockNumber>;
}

/// Transfer some liquid free balance to another account.
///
/// `transfer` will set the `FreeBalance` of the sender and receiver.
/// It will decrease the total issuance of the system by the `TransferFee`.
/// If the sender's account is below the existential deposit as a result
/// of the transfer, the account will be reaped.
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct TransferCall<'a, T: Balances> {
    /// Destination of the transfer.
    pub to: &'a <T as System>::Address,
    /// Amount to transfer.
    #[codec(compact)]
    pub amount: T::Balance,
}
