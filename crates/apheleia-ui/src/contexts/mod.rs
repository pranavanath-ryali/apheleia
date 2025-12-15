use apheleia_core::types::vector::Vector2;

pub struct RenderContext {
    pub position: Vector2,
    pub size: Vector2,
}

pub struct UpdateContext {
    pub position: Vector2,
    pub size: Option<Vector2>,
}
