mod pool;
mod distribute_method;
mod peer;
mod power;
mod workload;
mod database;


pub use peer::Peer;
pub use power::Power;
pub use workload::Workload;
pub use pool::MiningPool;
pub use database::AccessDatabase;


pub trait Pool
{
    type Peers: Peer;
    type Workloads: Workload;
    type Powers: Power;
    type Database: AccessDatabase<<Self::Peers as Peer>::Id, <Self::Peers as Peer>::PeerInfo>;
    type MiningPool;

    /// Record peer's hash power
    fn record_peer_info(peer: &Self::Peers, database: &Self::Database);

    /// Receive new work, sign it among peers
    /// calling `fn sign_work_to()` according to `distribute_method`
    fn add_new_work(&mut self, work: Self::Workloads, mining_pool: &mut Self::MiningPool);

    /// Receive valid block from peer, add reset peer's status
    fn finish_job(peer_id: <Self::Peers as Peer>::Id, height: u32, mining_pool: &mut Self::MiningPool);
}
