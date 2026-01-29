use crate::Context;

use std::collections::HashMap;

pub fn resolve(ctx: &Context) -> String {
    // Currently, we only have one mode.
    // In the future, this is where you would put:
    // if cpu > 50 { return "style_mecha" }
    
    if let Some(config) = ctx.data.get("style_peace") {
        return config.to_string();
    }

    "# Error: style_peace not found".to_string()
}
