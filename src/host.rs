use crate::consensus_client::{
    ConsensusClient, ConsensusClientId, StateCommitment, StateMachineHeight, StateMachineId,
};
use crate::error::Error;
use crate::prelude::Vec;
use alloc::boxed::Box;
use codec::{Decode, Encode};
use core::time::Duration;
use derive_more::Display;

#[derive(Clone, Debug, Encode, Decode, Display)]
pub enum ChainID {
    #[codec(index = 0)]
    ETHEREUM,
    #[codec(index = 1)]
    GNOSIS,
    #[codec(index = 2)]
    ARBITRUM,
    #[codec(index = 3)]
    OPTIMISM,
    #[codec(index = 4)]
    BASE,
    #[codec(index = 5)]
    MOONBEAM,
    #[codec(index = 6)]
    ASTAR,
    #[codec(index = 7)]
    HYPERSPACE,
}

pub trait ISMPHost {
    fn host(&self) -> ChainID;

    // Storage Read functions

    /// Returns the latest height of the state machine
    fn latest_commitment_height(&self, id: StateMachineId) -> Result<StateMachineHeight, Error>;
    /// Returns the state machine at the give height
    fn state_machine_commitment(
        &self,
        height: StateMachineHeight,
    ) -> Result<StateCommitment, Error>;
    /// Returns the host timestamp when this consensus client was last updated
    fn consensus_update_time(&self, id: ConsensusClientId) -> Result<Duration, Error>;
    /// Returns the host timestamp when this consensus client was updated
    fn state_machine_update_time(&self, height: StateMachineHeight) -> Result<Duration, Error>;
    /// Returns the scale encoded consensus state for a consensus client
    fn consensus_state(&self, id: ConsensusClientId) -> Result<Vec<u8>, Error>;
    /// Return the host timestamp in nanoseconds
    fn host_timestamp(&self) -> Duration;

    // Storage Write functions

    /// Store a scale encoded consensus state
    fn store_consensus_state(&self, id: ConsensusClientId, state: Vec<u8>) -> Result<(), Error>;
    /// Store the timestamp when the consensus client was updated
    fn store_consensus_update_time(
        &self,
        id: ConsensusClientId,
        timestamp: Duration,
    ) -> Result<(), Error>;
    /// Store the timestamp when the state machine was updated
    fn store_state_machine_update_time(
        &self,
        height: StateMachineHeight,
        timestamp: Duration,
    ) -> Result<(), Error>;


    /// Should return a handle to the consensus client based on the id
    fn consensus_client(&self, id: ConsensusClientId) -> Box<dyn ConsensusClient>;
}
