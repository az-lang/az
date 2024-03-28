use std::rc::Rc;
use std::sync::Arc;

use super::byte_index::ByteIndex;

pub trait ByteSize {
    fn byte_size(&self) -> ByteIndex;
}

impl ByteSize for char {
    fn byte_size(&self) -> ByteIndex {
        self.len_utf8().into()
    }
}

impl ByteSize for &str {
    fn byte_size(&self) -> ByteIndex {
        self.len().into()
    }
}

macro_rules! impl_byte_size {
    ($string_type:ty) => {
        impl ByteSize for $string_type {
            fn byte_size(&self) -> ByteIndex {
                AsRef::<str>::as_ref(self).byte_size()
            }
        }
    };
}

impl_byte_size!(Arc<str>);
impl_byte_size!(Box<str>);
impl_byte_size!(String);
impl_byte_size!(Rc<str>);
