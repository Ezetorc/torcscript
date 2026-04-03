use std::collections::HashMap;

use crate::interpreter::{
    native_members::list::properties::list_size, native_property::NativeProperty,
};

pub fn get_list_properties() -> HashMap<&'static str, NativeProperty> {
    let mut props: HashMap<&str, NativeProperty> = HashMap::new();

    props.insert("size", list_size);

    props
}
