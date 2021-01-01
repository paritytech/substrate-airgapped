// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generic implementation of an unchecked (pre-verification) extrinsic.

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, Error, Input, Output};

/// Era period
pub type Period = u64;

/// Era phase
pub type Phase = u64;

/// An era to describe the longevity of a transaction.
///
/// equivalent of `sp_runtime::generic::Era`
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Era {
	/// The transaction is valid forever. The genesis hash must be present in the signed content.
	Immortal,

	/// Period and phase are encoded:
	/// - The period of validity from the block hash found in the signing material.
	/// - The phase in the period that this transaction's lifetime begins (and, importantly,
	/// implies which block hash is included in the signature material). If the `period` is
	/// greater than 1 << 12, then it will be a factor of the times greater than 1<<12 that
	/// `period` is.
	///
	/// When used on `FRAME`-based runtimes, `period` cannot exceed `BlockHashCount` parameter
	/// of `system` module.
	Mortal(Period, Phase),
}

impl Era {
	/// Create a new era based on a period (which should be a power of two between 4 and 65536 inclusive)
	/// and a block number on which it should start (or, for long periods, be shortly after the start).
	///
	/// If using `Era` in the context of `FRAME` runtime, make sure that `period`
	/// does not exceed `BlockHashCount` parameter passed to `system` module, since that
	/// prunes old blocks and renders transactions immediately invalid.
	pub fn mortal(period: u64, current: u64) -> Self {
		let period = period.checked_next_power_of_two().unwrap_or(1 << 16).max(4).min(1 << 16);
		let phase = current % period;
		let quantize_factor = (period >> 12).max(1);
		let quantized_phase = phase / quantize_factor * quantize_factor;

		Era::Mortal(period, quantized_phase)
	}

	/// Create an "immortal" transaction.
	pub fn immortal() -> Self {
		Era::Immortal
	}

	/// `true` if this is an immortal transaction.
	pub fn is_immortal(&self) -> bool {
		match self {
			Era::Immortal => true,
			_ => false,
		}
	}
}

impl Encode for Era {
	fn encode_to<T: Output>(&self, output: &mut T) {
		match self {
			Era::Immortal => output.push_byte(0),
			Era::Mortal(period, phase) => {
				let quantize_factor = (*period as u64 >> 12).max(1);
				let encoded = (period.trailing_zeros() - 1).max(1).min(15) as u16
					| ((phase / quantize_factor) << 4) as u16;
				output.push(&encoded);
			}
		}
	}
}

impl Decode for Era {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		let first = input.read_byte()?;
		if first == 0 {
			Ok(Era::Immortal)
		} else {
			let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
			let period = 2 << (encoded % (1 << 4));
			let quantize_factor = (period >> 12).max(1);
			let phase = (encoded >> 4) * quantize_factor;
			if period >= 4 && phase < period {
				Ok(Era::Mortal(period, phase))
			} else {
				Err("Invalid period and phase".into())
			}
		}
	}
}
