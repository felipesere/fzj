use anyhow::{Context, Result, bail};
use bpaf::Bpaf;
use dialoguer::{FuzzySelect, console::Term, theme::ColorfulTheme};
use serde_json::Value;
use std::{
    io::{self, ErrorKind, Read},
    process,
};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Options {
    /// Comma-separated list of fields to display during selection
    #[bpaf(long("fields"), argument("FIELDS"), optional)]
    fields: Option<String>,

    /// JSON path to extract nested array (e.g., "data.items")
    #[bpaf(long("dig"), argument("PATH"), optional)]
    dig: Option<String>,

    /// Comma-separated list of fields to output in the final result
    #[bpaf(long("out"), argument("FIELDS"), optional)]
    out: Option<String>,
}

fn main() -> Result<()> {
    let opts = options().run();

    let _ = ctrlc::set_handler(move || {
        // this is oddly load-bearing to reset the curser after hitting control-c
    });

    // Read JSON from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read from stdin")?;

    // Parse JSON
    let mut json: Value = serde_json::from_str(&input).context("Failed to parse JSON")?;

    // Extract nested array if --dig is specified
    if let Some(path) = &opts.dig {
        json = dig_path(&json, path).context("Failed to extract path with --dig")?;
    }

    // Ensure we have an array
    let array = json
        .as_array()
        .context("Input must be a JSON array (or use --dig to extract an array)")?;

    if array.is_empty() {
        anyhow::bail!("Input array is empty");
    }

    // Parse field lists
    let display_fields = opts.fields.as_ref().map(|f| parse_field_list(f));
    let output_fields = opts.out.as_ref().map(|f| parse_field_list(f));

    // Create display strings for each item
    let items: Vec<String> = array
        .iter()
        .map(|item| format_item(item, display_fields.as_ref()))
        .collect();

    // Show fuzzy selection dialog
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an item:")
        .items(&items)
        .default(0)
        .highlight_matches(true)
        .interact_opt();

    let selection = match selection {
        Ok(Some(selection)) => selection,
        Ok(None) => {
            let _ = Term::stderr().show_cursor();
            process::exit(1);
        }
        Err(dialoguer::Error::IO(io_err)) => {
            let _ = Term::stderr().show_cursor();
            if io_err.kind() == ErrorKind::Interrupted {
                process::exit(1);
            }
            bail!("{io_err:?}")
        }
    };

    // Get the selected JSON object
    let selected = &array[selection];

    // Filter output fields if specified
    let output = if let Some(fields) = output_fields {
        filter_fields(selected, &fields)?
    } else {
        selected.clone()
    };

    // Print the result as JSON
    println!("{}", serde_json::to_string(&output)?);

    Ok(())
}

fn parse_field_list(fields: &str) -> Vec<String> {
    fields
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn format_item(item: &Value, fields: Option<&Vec<String>>) -> String {
    match item {
        Value::Object(map) => {
            let fields_to_show = if let Some(field_list) = fields {
                field_list.clone()
            } else {
                map.keys().cloned().collect()
            };

            let parts: Vec<String> = fields_to_show
                .iter()
                .filter_map(|key| {
                    map.get(key).map(|value| {
                        let formatted = match value {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            Value::Null => "null".to_string(),
                            Value::Array(_) => "[...]".to_string(),
                            Value::Object(_) => "{...}".to_string(),
                        };
                        format!("{}: {}", key, formatted)
                    })
                })
                .collect();

            if parts.is_empty() {
                "{empty}".to_string()
            } else {
                parts.join(", ")
            }
        }
        _ => format!("{}", item),
    }
}

fn dig_path(value: &Value, path: &str) -> Result<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;

    for part in parts {
        current = current
            .get(part)
            .with_context(|| format!("Path component '{}' not found", part))?;
    }

    Ok(current.clone())
}

fn filter_fields(value: &Value, fields: &[String]) -> Result<Value> {
    match value {
        Value::Object(map) => {
            let mut filtered = serde_json::Map::new();
            for field in fields {
                if let Some(val) = map.get(field) {
                    filtered.insert(field.clone(), val.clone());
                }
            }
            Ok(Value::Object(filtered))
        }
        _ => anyhow::bail!("Cannot filter fields on non-object value"),
    }
}
