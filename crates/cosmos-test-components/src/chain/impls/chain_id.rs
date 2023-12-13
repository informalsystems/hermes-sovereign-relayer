use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_test_components::chain::traits::build::ChainIdFromStringBuilder;

pub struct BuildCosmosChainIdFromString;

impl<Chain> ChainIdFromStringBuilder<Chain> for BuildCosmosChainIdFromString
where
    Chain: HasChainIdType<ChainId = ChainId>,
{
    fn build_chain_id_from_string(chain_id: &str) -> ChainId {
        ChainId::from_string(chain_id)
    }
}
