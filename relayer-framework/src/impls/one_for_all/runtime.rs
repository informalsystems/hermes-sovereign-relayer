use crate::impls::one_for_all::error::OfaErrorContext;
use crate::traits::core::ErrorContext;
use crate::traits::one_for_all::runtime::OfaRuntime;

pub struct OfaRuntimeContext<Runtime> {
    pub runtime: Runtime,
}

impl<Runtime: OfaRuntime> ErrorContext for OfaRuntimeContext<Runtime> {
    type Error = OfaErrorContext<Runtime::Error>;
}
