use core::{fmt, num::TryFromIntError};
use sp_runtime::transaction_validity::TransactionValidityError;

/// Descriptive error type
#[cfg(feature = "std")]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error(&'static str);

/// Un-descriptive error type when compiled for no std
#[cfg(not(feature = "std"))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error;

impl Error {
	/// Error description
	///
	/// This function returns an actual error str when running in `std`
	/// environment, but `""` on `no_std`.
	#[cfg(feature = "std")]
	pub fn what(&self) -> &'static str {
		self.0
	}

	/// Error description
	///
	/// This function returns an actual error str when running in `std`
	/// environment, but `""` on `no_std`.
	#[cfg(not(feature = "std"))]
	pub fn what(&self) -> &'static str {
		""
	}
}

#[cfg(feature = "std")]
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(not(feature = "std"))]
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Error")
	}
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
	fn source(&self) -> &str {
		self.0
	}
}

impl From<&'static str> for Error {
	#[cfg(feature = "std")]
	fn from(s: &'static str) -> Error {
		Error(s)
	}

	#[cfg(not(feature = "std"))]
	fn from(_s: &'static str) -> Error {
		Error
	}
}

impl From<TransactionValidityError> for Error {
	#[cfg(feature = "std")]
	fn from(t: TransactionValidityError) -> Error {
		Error(t.into())
	}

	#[cfg(not(feature = "std"))]
	fn from(_t: TransactionValidityError) -> Error {
		Error
	}
}

impl From<TryFromIntError> for Error {
	#[cfg(feature = "std")]
	fn from(t: TryFromIntError) -> Error {
		Error(t.into())
	}

	#[cfg(not(feature = "std"))]
	fn from(_t: TryFromIntError) -> Error {
		Error
	}
}
