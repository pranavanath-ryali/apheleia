use apheleia_types::{point::Point, size::Size};

#[derive(Clone)]
pub struct NodeData {
    pub position: Point<i32>,
    pub size: Size<u16>,
}
