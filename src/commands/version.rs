use anyhow::Result;

pub fn get_version() -> Result<()> {
    // Get the version from Cargo.toml at compile time
    println!("Version: v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}