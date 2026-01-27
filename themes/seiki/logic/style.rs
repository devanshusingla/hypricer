use std::collections::HashMap;

pub fn apply(inputs: HashMap<String, String>) -> String {
    // Basic test: Return a static blue border
    return "general { col.active_border = rgba(33ccffee) }".to_string();
}
