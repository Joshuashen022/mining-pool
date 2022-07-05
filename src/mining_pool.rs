use std::collections::HashMap;
use crate::distribute_method::DistributeMethod;

#[derive(Default)]
pub struct MiningPool<Peer, Power, Workload>
where
    Peer: Default,
    Power: Default,
    Workload: Default,
{
    /// Contains all the peers of the mining pool network
    /// it changes whenever a peer join or leave the mining pool
    peers: HashMap<Peer, Power>,

    /// Keep the track of the total hash power of the network
    total_power: Power,

    /// Current total workload
    total_workload: Workload,

    /// Current work distribution.
    /// It changes whenever:
    /// 1) A peer join the mining pool,
    /// 2) A peer leave the mining pool,
    /// 3) New work has been sent to the mining pool.
    current_work_distribution: HashMap<Peer, Workload>,

    /// Way to distribute hash power
    distribute_method: DistributeMethod,
}

impl<Peer, Power, Workload> MiningPool<Peer, Power, Workload>
where
    Peer: Default,
    Power: Default,
    Workload: Default,
{
    pub fn increase_work_load(){}

    pub fn add_peer(){}

    pub fn peer_leave(){}

    pub fn add_new_work(){}

}