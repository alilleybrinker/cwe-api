//! Handles printing output to the user.

use crate::cli::Format;
use anyhow::Result;
use minijinja::{context, Environment};
use serde_json::Value as JsonValue;
use tracing::debug;

/// Report output.
pub fn report(format: Format, template_name: &str, value: JsonValue) -> Result<()> {
    match format {
        Format::Json => report_json(value),
        Format::Human => report_human(template_name, value),
    }
}

/// Report JSON-format output.
fn report_json(value: JsonValue) -> Result<()> {
    let to_print = serde_json::to_string_pretty(&value)?;
    println!("{}", to_print);
    Ok(())
}

/// Report human-readable output.
fn report_human(template_name: &str, value: JsonValue) -> Result<()> {
    // Load the template environment.
    let mut env = Environment::new();
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(false);
    minijinja_embed::load_templates!(&mut env);

    // Load the specific template we need.
    let template_name = format!("{}.jinja", template_name);
    debug!(template_name = %template_name);
    let template = env.get_template(&template_name)?;

    // Set up the template context.
    let context = context!(context => value);
    debug!(context = ?context);

    // Render and print the template.
    let rendered = template.render(context)?;
    println!("{}", rendered);

    Ok(())
}
