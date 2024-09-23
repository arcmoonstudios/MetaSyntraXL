// build.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[BUILD]Xyn>=====S===t===u====d===i===o===s====[R|$>

use walkdir::{DirEntry, WalkDir};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=CONFIG_PATH");

    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_string());

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f = File::create(&dest_path).expect("Could not create config.rs");
    writeln!(f, "pub const CONFIG_PATH: &str = \"{}\";", config_path)
        .expect("Could not write to config.rs");

    add_custom_headers(&env::current_dir()?)?;

    Ok(())
}

fn add_custom_headers(project_path: &Path) -> Result<()> {
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_entry(|e| should_include(e))
    {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if let Some(header) = generate_custom_header(project_path, path, extension) {
                    add_header_to_file(path, &header)?;
                }
            }
        }
    }
    Ok(())
}

fn should_include(entry: &DirEntry) -> bool {
    let path = entry.path();
    if path.is_dir() {
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            // Exclude 'target' and hidden directories
            return !(file_name == "target" || file_name.starts_with('.'));
        }
    }
    true
}

fn generate_custom_header(project_root: &Path, path: &Path, extension: &str) -> Option<String> {
    if extension == "toml" {
        return None;
    }

    let relative_path = path.strip_prefix(project_root).unwrap_or(path);
    let components: Vec<_> = relative_path.components().collect();
    
    let module_name = if components.len() == 1 || (components.len() == 2 && components[0].as_os_str() == "src") {
        // File is in the root or directly under src
        path.file_stem().unwrap().to_string_lossy().to_uppercase()
    } else {
        // File is in a subdirectory
        components[components.len() - 2].as_os_str().to_string_lossy().to_uppercase()
    };

    // Replace underscores with hyphens
    let module_name = module_name.replace('_', "-");

    let comment_syntax = match extension {
        "rs" => "//",
        "py" | "sh" | "bash" => "#",
        "js" | "ts" | "jsx" | "tsx" | "css" | "scss" => "//",
        "html" | "xml" => "<!--",
        "sql" => "--",
        "md" => "#",
        _ => return None,
    };

    let display_path = relative_path.display().to_string();

    let header = format!(
        comment_syntax,
        display_path,
        module_name
    );

    Some(header)
}

fn add_header_to_file(path: &Path, new_header: &str) -> Result<()> {
    let mut content = String::new();
    {
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;
    }

    // Remove all existing headers
    let cleaned_content: String = content
        .lines()
        .collect::<Vec<&str>>()
        .join("\n");

    // Add the new header
    let new_content = format!("{}{}", new_header, cleaned_content);

    // Write the new content back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}