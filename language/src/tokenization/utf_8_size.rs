use std::rc::Rc;
use std::sync::Arc;

use super::utf_8_index::Utf8Index;

pub trait Utf8Size {
    fn utf_8_size(&self) -> Utf8Index;
}

impl Utf8Size for char {
    fn utf_8_size(&self) -> Utf8Index {
        1usize.into()
    }
}

impl Utf8Size for str {
    fn utf_8_size(&self) -> Utf8Index {
        self.chars().count().into()
    }
}

macro_rules! impl_utf_8_size {
    ($string_type:ty) => {
        impl Utf8Size for $string_type {
            fn utf_8_size(&self) -> Utf8Index {
                AsRef::<str>::as_ref(self).utf_8_size()
            }
        }
    };
}

impl_utf_8_size!(Arc<str>);
impl_utf_8_size!(Box<str>);
impl_utf_8_size!(String);
impl_utf_8_size!(Rc<str>);
