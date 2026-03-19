use crate::NodeId;

pub struct NodeIdGenerator {
    id: NodeId,
}
impl Default for NodeIdGenerator {
    fn default() -> Self {
        NodeIdGenerator { id: 0 }
    }
}
impl NodeIdGenerator {
    pub fn next(&mut self) -> NodeId {
        self.id += 1;
        self.id
    }
}
