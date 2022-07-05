
#[derive(Clone, Debug)]
pub enum DistributeMethod{
    /// The allocated workload of a peer percentage
    /// is same as its hash power percentage
    Default,

    /// First set peer to different groups (by hash power or other things),
    /// then allocate the group with the same workload
    Trunk,
}
impl Default for DistributeMethod{
    fn default() -> Self {
        DistributeMethod::Default
    }
}