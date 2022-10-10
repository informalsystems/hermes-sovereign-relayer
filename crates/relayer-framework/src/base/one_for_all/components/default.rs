use crate::base::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use crate::base::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use crate::base::impls::messages::skip_update_client::SkipUpdateClient;
use crate::base::impls::messages::wait_update_client::WaitUpdateClient;
use crate::base::impls::packet_relayers::top::TopRelayer;
use crate::base::one_for_all::impls::chain::OfaConsensusStateQuerier;
use crate::base::one_for_all::impls::relay::OfaUpdateClientMessageBuilder;
use crate::base::one_for_all::impls::status::OfaChainStatusQuerier;
use crate::base::one_for_all::traits::chain::{OfaChain, OfaIbcChain};
use crate::base::one_for_all::traits::components::chain::{
    OfaChainComponents, OfaIbcChainComponents,
};
use crate::base::one_for_all::traits::components::relay::OfaRelayComponents;
use crate::base::one_for_all::traits::relay::OfaRelay;

pub struct DefaultComponents;

impl<Chain> OfaChainComponents<Chain> for DefaultComponents
where
    Chain: OfaChain,
{
    type ChainStatusQuerier = OfaChainStatusQuerier;
}

impl<Chain, Counterparty> OfaIbcChainComponents<Chain, Counterparty> for DefaultComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaIbcChain<Chain>,
{
    type ConsensusStateQuerier = OfaConsensusStateQuerier;
}

impl<Relay> OfaRelayComponents<Relay> for DefaultComponents
where
    Relay: OfaRelay<Components = DefaultComponents>,
{
    type PacketRelayer = TopRelayer;

    type UpdateClientMessageBuilder =
        SkipUpdateClient<WaitUpdateClient<OfaUpdateClientMessageBuilder>>;

    type IbcMessageSender = SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>;
}