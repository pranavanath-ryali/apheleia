use std::any::Any;

pub trait Extension: Any {
    fn as_any(&self) -> &dyn Any;
}
