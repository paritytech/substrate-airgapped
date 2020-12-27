use codec::Decode;

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

/// Runtime trait.
pub trait Runtime: System + Sized + Send + Sync + 'static {
    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;
    /// Transaction extras.
    type Extra: SignedExtra<Self> + Send + Sync + 'static;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolkadotRuntime;

/// UncheckedExtrinsic type.
pub type UncheckedExtrinsic<T> = sp_runtime::generic::UncheckedExtrinsic<
    <T as System>::Address,
    Encoded,
    <T as Runtime>::Signature,
    Extra<T>,
>;

pub fn decode_unchecked_extrinsic(unchecked_extrinsic: &mut b[..]) -> UncheckedExtrinsic {

}
