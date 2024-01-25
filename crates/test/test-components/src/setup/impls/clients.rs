use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientId;
use hermes_relayer_components::relay::traits::chains::CanRaiseRelayChainErrors;
use hermes_relayer_components::relay::traits::components::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::relay_at::{HasRelayTypeAt, RelayTypeAt};
use crate::setup::traits::clients::ClientSetup;
use crate::setup::traits::create_client_options_at::HasCreateClientOptionsAt;
use crate::types::index::Twindex;

pub struct SetupClientsWithRelay;

impl<Setup, const A: usize, const B: usize> ClientSetup<Setup, A, B> for SetupClientsWithRelay
where
    Setup: HasErrorType
        + HasRelayTypeAt<A, B>
        + HasCreateClientOptionsAt<A, B>
        + HasCreateClientOptionsAt<B, A>
        + CanRaiseError<<RelayTypeAt<Setup, A, B> as HasErrorType>::Error>,
    ChainTypeAt<Setup, A>:
        HasIbcChainTypes<ChainTypeAt<Setup, B>> + HasCreateClientOptionsType<ChainTypeAt<Setup, B>>,
    ChainTypeAt<Setup, B>:
        HasIbcChainTypes<ChainTypeAt<Setup, A>> + HasCreateClientOptionsType<ChainTypeAt<Setup, A>>,
    RelayTypeAt<Setup, A, B>: CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>
        + CanRaiseRelayChainErrors,
{
    async fn setup_clients(
        setup: &Setup,
        chain_a: &ChainTypeAt<Setup, A>,
        chain_b: &ChainTypeAt<Setup, B>,
    ) -> Result<
        (
            ClientId<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
            ClientId<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
        ),
        Setup::Error,
    > {
        let client_id_a = <RelayTypeAt<Setup, A, B>>::create_client(
            SourceTarget,
            chain_a,
            chain_b,
            setup.create_client_options(Twindex::<B, A>),
        )
        .await
        .map_err(Setup::raise_error)?;

        let client_id_b = <RelayTypeAt<Setup, A, B>>::create_client(
            DestinationTarget,
            chain_b,
            chain_a,
            setup.create_client_options(Twindex::<A, B>),
        )
        .await
        .map_err(Setup::raise_error)?;

        Ok((client_id_a, client_id_b))
    }
}