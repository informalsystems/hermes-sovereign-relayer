use alloc::vec;

use cgp_core::{async_trait, CanRun, HasErrorType, Runner};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};

/// A concurrent two-way relay context that is composed of a `BiRelay` type that
/// can auto-relay between two connected targets.
///
/// As opposed to the `ParallelTwoWayAutoRelay` variant, this concurrent variant
/// runs in a single thread and achieves concurrency via cooperative multi-tasking.
pub struct RelayBothWays;

pub enum TwoWayRelayerTask<BiRelay>
where
    BiRelay: HasTwoWayRelay,
{
    AToB(BiRelay::RelayAToB),
    BToA(BiRelay::RelayBToA),
}

#[async_trait]
impl<BiRelay> Task for TwoWayRelayerTask<BiRelay>
where
    BiRelay: HasTwoWayRelay,
    BiRelay::RelayAToB: CanRun,
    BiRelay::RelayBToA: CanRun,
{
    async fn run(self) {
        match self {
            Self::AToB(relay) => {
                let _ = relay.run().await;
            }
            Self::BToA(relay) => {
                let _ = relay.run().await;
            }
        }
    }
}

#[async_trait]
impl<BiRelay> Runner<BiRelay> for RelayBothWays
where
    BiRelay: HasTwoWayRelay + HasRuntime + HasErrorType,
    BiRelay::RelayAToB: Clone + CanRun,
    BiRelay::RelayBToA: Clone + CanRun,
    BiRelay::Runtime: CanRunConcurrentTasks,
{
    async fn run(birelay: &BiRelay) -> Result<(), BiRelay::Error> {
        let tasks = vec![
            <TwoWayRelayerTask<BiRelay>>::AToB(birelay.relay_a_to_b().clone()),
            <TwoWayRelayerTask<BiRelay>>::BToA(birelay.relay_b_to_a().clone()),
        ];

        birelay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
