use std::any::Any;

pub trait Resource: Any {
    fn as_any(&self) -> &dyn Any;
}
