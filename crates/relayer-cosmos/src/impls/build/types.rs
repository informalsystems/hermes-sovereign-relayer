use cgp_core::traits::HasErrorType;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_components::build::traits::birelay::HasBiRelayType;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::Error as TokioError;
use ibc_relayer_runtime::types::log::logger::TracingLogger;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::types::error::{BaseError, Error};

impl HasBiRelayType for CosmosBuilder {
    type BiRelay = CosmosBiRelay<BaseChainHandle, BaseChainHandle>;

    fn birelay_error(e: Error) -> Error {
        e
    }
}

impl HasErrorType for CosmosBuilder {
    type Error = Error;
}

impl HasRuntime for CosmosBuilder {
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl HasLoggerType for CosmosBuilder {
    type Logger = TracingLogger;
}

impl HasLogger for CosmosBuilder {
    fn logger(&self) -> &TracingLogger {
        &TracingLogger
    }
}
