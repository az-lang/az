use std::rc::Rc;
use std::sync::Arc;

use super::utf_8_count::Utf8Count;

pub trait Utf8Size {
    fn utf_8_size(&self) -> Utf8Count;
}

impl Utf8Size for char {
    fn utf_8_size(&self) -> Utf8Count {
        1usize.into()
    }
}

impl Utf8Size for str {
    fn utf_8_size(&self) -> Utf8Count {
        self.chars().count().into()
    }
}

macro_rules! impl_utf_8_size {
    ($string_type:ty) => {
        impl Utf8Size for $string_type {
            fn utf_8_size(&self) -> Utf8Count {
                AsRef::<str>::as_ref(self).utf_8_size()
            }
        }
    };
}

impl_utf_8_size!(Arc<str>);
impl_utf_8_size!(Box<str>);
impl_utf_8_size!(String);
impl_utf_8_size!(Rc<str>);
