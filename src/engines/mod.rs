pub mod aws;
pub mod kv_v2;

use std::collections::HashMap;
pub fn hashmap_eq(lhs: &HashMap<String, String>, rhs: &HashMap<String, String>) -> bool {
    if lhs.len() != rhs.len() {return false;}
    for (key, value) in lhs.iter() {
        match rhs.get(key) {
            Some(rvalue) => {
                if !value.eq(rvalue) {return false;}
            }

            None => {return false;}
        }
    }
    true
}
