use crate::{stores::nodebuffer::NodeBufferStore, traits::resource::Resource};

/// Marks [`NodeBufferStore`] as a world [`Resource`].
impl Resource for NodeBufferStore {}
