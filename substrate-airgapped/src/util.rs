use core::fmt;

// Pretty print an integer with commas every three digits.
pub(crate) fn int_as_human<T: fmt::Display>(bal: T) -> String {
	let mut pretty_bal = String::new();
	let bal_str = bal.to_string();
	for (idx, val) in bal_str.chars().rev().enumerate() {
		if idx != 0 && idx % 3 == 0 {
			pretty_bal.insert(0, ',');
		}
		pretty_bal.insert(0, val);
	}

	pretty_bal
}

/// Simple wrapper to display hex representation of bytes.
///
/// Same as `sp_core::hexdisplay::HexDisplay`. Re-defined to make it easier to remove dependance on `sp-core`.
pub(crate) struct HexDisplay<'a>(&'a [u8]);

impl<'a> HexDisplay<'a> {
	/// Create new instance that will display `d` as a hex string when displayed.
	pub fn from<R: AsBytesRef>(d: &'a R) -> Self {
		HexDisplay(d.as_bytes_ref())
	}
}

impl<'a> core::fmt::Display for HexDisplay<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		for byte in self.0 {
			f.write_fmt(format_args!("{:02x}", byte))?;
		}

		Ok(())
	}
}

/// Simple trait to transform various types to `&[u8]`
///
/// Same as `sp_core::hexdisplay::AsBytesRef`. Re-defined to make it easier to remove dependance on `sp-core`.
pub(crate) trait AsBytesRef {
	/// Transform `self` into `&[u8]`.
	fn as_bytes_ref(&self) -> &[u8];
}

impl AsBytesRef for &[u8] {
	fn as_bytes_ref(&self) -> &[u8] {
		self
	}
}

impl AsBytesRef for [u8] {
	fn as_bytes_ref(&self) -> &[u8] {
		&self
	}
}

impl AsBytesRef for Vec<u8> {
	fn as_bytes_ref(&self) -> &[u8] {
		&self
	}
}
