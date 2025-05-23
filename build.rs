use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Parse DATABASE_URL from environment
    let db_path = if let Ok(db_url) = env::var("DATABASE_URL") {
        // Extract path from DATABASE_URL (format: sqlite://path/to/file)
        if let Some(path) = db_url.strip_prefix("sqlite://") {
            println!("cargo:warning=Database path set to: {}", path);
            path.to_string()
        } else {
            println!("cargo:warning=DATABASE_URL does not have expected format 'sqlite://path/to/file': {}", db_url);
            "./db/shop.db".to_string() // Default fallback
        }
    } else {
        println!("cargo:warning=DATABASE_URL environment variable not found, using default");
        "./db/shop.db".to_string() // Default fallback
    };

    // Create the directory if it doesn't exist
    let path = Path::new(&db_path);
    if let Some(dir) = path.parent() {
        if !dir.exists() {
            println!("cargo:warning=Creating database directory: {:?}", dir);
            fs::create_dir_all(dir).unwrap_or_else(|e| {
                println!("cargo:warning=Failed to create database directory: {}", e);
            });
        }

        // Ensure directory permissions are set correctly on Unix
        #[cfg(unix)]
        set_permissions(dir, 0o755);
    }

    // Check if the database file exists, if not create an empty SQLite database
    if !path.exists() {
        println!(
            "cargo:warning=Creating empty SQLite database file at: {:?}",
            path
        );

        // Use sqlite3 command line to create an empty database with schema
        match Command::new("sqlite3")
            .arg(&db_path)
            .arg("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, price REAL NOT NULL, created_at INTEGER NOT NULL DEFAULT (unixepoch()));")
            .status() {
                Ok(status) => {
                    if status.success() {
                        println!("cargo:warning=Successfully created SQLite database");
                    } else {
                        println!("cargo:warning=Failed to create SQLite database, exit code: {:?}", status);
                    }
                },
                Err(e) => {
                    println!("cargo:warning=Failed to execute sqlite3 command: {}", e);
                    // Fallback: create empty file
                    match fs::File::create(path) {
                        Ok(_) => println!("cargo:warning=Created empty database file (without schema)"),
                        Err(e) => println!("cargo:warning=Failed to create empty database file: {}", e)
                    }
                }
            }

        // Set appropriate permissions for the database file on Unix
        #[cfg(unix)]
        set_permissions(path, 0o644);
    }
}

// Helper function to set file permissions on Unix
#[cfg(unix)]
fn set_permissions(path: &Path, mode: u32) {
    if let Ok(metadata) = fs::metadata(path) {
        let mut perms = metadata.permissions();
        perms.set_mode(mode);
        if let Err(e) = fs::set_permissions(path, perms) {
            println!(
                "cargo:warning=Failed to set permissions on {:?}: {}",
                path, e
            );
        } else {
            println!("cargo:warning=Set permissions {:o} on {:?}", mode, path);
        }
    }
}
