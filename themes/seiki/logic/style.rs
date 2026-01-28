use crate::Context;

use std::collections::HashMap;

pub fn resolve(ctx: &Context) -> String {
    // Simple test: return static blue border
    "general { col.active_border = rgba(33ccffee) }".to_string()
}
