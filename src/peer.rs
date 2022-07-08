pub trait Peer<P = Self>
{
    type Id;
    type PeerInfo;

    /// Read physical information of this PC
    fn gather_info() -> Self;

}