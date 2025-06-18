use serde::Deserialize;
use std::fs;
use anyhow::{Result, Context};
use dirs::home_dir;
use clap::Clap;

/// Command-line interface arguments.
pub struct Cli {
    /// Host address for the Snapcast server.
    #[clap(short = 'H', long, global = true, default_value = "127.0.0.1", env = "SNAPSERVER_HOST")]
    pub host: String,

    /// Port number for the Snapcast server.
    #[clap(short, long, global = true, default_value = "1780", env = "SNAPSERVER_PORT")]
    pub port: u16,

    /// Command to execute.
    #[clap(subcommand)]
    pub command: Command,
}

/// Arguments for the delete command.
pub struct DeleteArgs {
    /// Subcommand for the delete command.
    #[clap(subcommand)]
    pub subcommand: DeleteSubcommand,
}

/// Arguments for the get command.
pub struct GetArgs {
    /// Subcommand for the get command.
    #[clap(subcommand)]
    pub subcommand: GetSubcommand,
}

/// Arguments for the set command.
pub struct SetArgs {
    /// Subcommand for the set command.
    #[clap(subcommand)]
    pub subcommand: SetSubcommand,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    // Fields removed as we're using CLI defaults
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        let config_content = fs::read_to_string(&config_path)?;
        let config: Config = serde_yaml::from_str(&config_content)
            .with_context(|| format!("Failed to parse config file at {:?}", config_path))?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

fn get_config_path() -> Result<std::path::PathBuf> {
    home_dir()
        .map(|mut path| {
            path.push(".config/snapctl/config.yaml");
            path
        })
        .with_context(|| "Failed to determine home directory")
        .ok_or_else(|| anyhow::anyhow!("Home directory not found"))
}