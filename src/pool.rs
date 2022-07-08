use std::collections::HashMap;
use std::hash::Hash;

use crate::peer::Peer;
use crate::power::Power;
use crate::workload::Workload;
use crate::database::AccessDatabase;
use crate::distribute_method::DistributeMethod;

#[derive(Default)]
pub struct MiningPool<PeersId, Powers, Workloads, Database>
where
    PeersId: Default,
    Powers: Default ,
    Workloads: Default,
    Database: AccessDatabase<PeersId, Workloads>,

{
    /// Contains all the peers of the mining pool network
    /// it changes whenever a peer join or leave the mining pool
    peers: HashMap<PeersId, Powers>,

    /// Keep the track of the total hash power of the network
    total_power: Powers,

    /// Current total workload
    total_workload: Workloads,

    /// Current work distribution.
    /// It changes whenever:
    /// 1) A peer join the mining pool,
    /// 2) A peer leave the mining pool,
    /// 3) New work has been sent to the mining pool.
    current_work_distribution: HashMap<PeersId, Workloads>,

    /// Way to distribute hash power
    distribute_method: DistributeMethod,

    /// Keep a track of the work peer has finished
    finished_work: Database,
}


impl<PeersId, Powers, Workloads, Database> MiningPool<PeersId, Powers, Workloads, Database>
where
    PeersId: Default + Eq + Hash + AsRef<[u8]> + Clone,
    Powers: Default + Power + Clone,
    Workloads: Default + Workload + Clone + Ord,
    Database: AccessDatabase<PeersId, Workloads>,
{
    pub fn peers_total(&self) -> &HashMap<PeersId, Powers> {
        &self.peers
    }

    pub fn work_distribution(&self) -> &HashMap<PeersId, Workloads>{
        &self.current_work_distribution
    }

    /// Call this when a peer join the pool
    pub fn when_peer_join(&mut self, peer: PeersId, power: Powers){

        self.peers.insert(peer, power.clone());

        self.total_power.add(power);

    }

    /// Call this when a peer leave the pool
    pub fn when_peer_leave(&mut self,peer: PeersId){
        if let Some(power)= self.peers.remove(&peer){
            self.total_power.sub(power);
        };

    }

    /// Call this when sign a certain amount of work to a peer
    pub fn when_sign_work_to(&mut self, peer: PeersId, work: Workloads){
        self.current_work_distribution.insert(peer, work);
    }

    /// Call this method when peer finish some work.
    pub fn when_peer_finish_work(&mut self, peer: PeersId, finished_work: Workloads){
        if let Some(mut current_work) = self.current_work_distribution.remove(&peer){

            current_work.sub(finished_work.clone());
            self.current_work_distribution.insert(peer.clone(), current_work);

            if let Ok(mut work_data) = self.finished_work.get_value(&peer){
                work_data.add(finished_work);

                // Overwrite data base
                self.finished_work.put(&peer, &work_data).unwrap();
            }
        }
    }

    /// Return currently free peer but with highest hash rate
    pub fn get_peer_high_free(&self) -> Option<&PeersId>{
        if self.peers.len() == self.current_work_distribution.len() {
            return  None
        }

        assert!(self.peers.len() > self.current_work_distribution.len(),
                "Error! Working peer is more than total peer.");

        self.peers.keys()
            .filter(|peer|{
            self.current_work_distribution.keys()
                .find(|work_peer|work_peer == peer)
                .is_none()
        }).reduce(|a, b|{
            if self.current_work_distribution.get(a) > self.current_work_distribution.get(b) {
                a
            } else {
                b
            }
        })
    }

    /// Return currently less busy peer(not free) but with highest hash rate
    pub fn get_peer_high_less(&self) -> Option<&PeersId>{
        assert_eq!(self.peers.len(), self.current_work_distribution.len(),
                "Error! Working peers should be the same as total peers.");

        let res = self.peers
            .iter()
            .map(|(peer, power)|{
                if let Some(work) = self.current_work_distribution.get(peer){
                    let ratio = work.len() as f32 / power.measure() as f32;
                    (peer, ratio)
                } else {
                    let ratio = 0 as f32;
                    (peer, ratio)
                }
        }).reduce(|(peer1, ratio1), (peer2, ratio2)|{
            if ratio1 > ratio2{ (peer1, ratio1) } else { (peer2, ratio2) }
        });

        if let Some((peer, _)) = res{
            Some(peer)
        } else{
            None
        }
    }
}
