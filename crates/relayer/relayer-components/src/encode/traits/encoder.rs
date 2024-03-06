use cgp_core::prelude::*;

use crate::encode::traits::encoded::HasEncodedType;

#[derive_component(EncoderComponent, Encoder<Encoding>)]
pub trait CanEncode<Value>: HasEncodedType + HasErrorType {
    fn encode(&self, value: &Value) -> Result<Self::Encoded, Self::Error>;
}
