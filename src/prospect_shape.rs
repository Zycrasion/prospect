

use crate::abstraction::vertex::Vertex;

pub struct ProspectShape<VecList, IndexList>
    where   VecList : Into<Vec<Vertex>>,
            IndexList : Into<Vec<u16>>
{
    pub vertices : VecList,
    pub indices : Option<IndexList>,
}