use std::collections::HashMap;

use crate::interpreter::{
    native_members::list::methods::{
        list_add_element, list_get_element, list_get_element_at, list_remove_element,
    },
    native_method::NativeMethod,
};

pub fn get_list_methods() -> HashMap<&'static str, NativeMethod> {
    let mut methods: HashMap<&str, NativeMethod> = HashMap::new();

    methods.insert("add", list_add_element);
    methods.insert("remove", list_remove_element);
    methods.insert("get", list_get_element);
    methods.insert("get_at", list_get_element_at);

    methods
}
