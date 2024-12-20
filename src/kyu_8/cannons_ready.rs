use std::collections::HashMap;

pub fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    for (_, &value) in gunners.iter() {
        if value == "nay" {
            return "Shiver me timbers!".to_string();
        }
    }
    return "Fire!".to_string();
}