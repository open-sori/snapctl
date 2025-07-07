mod cli;
mod commands;
mod rpc;
mod utils;

use clap::Parser;
use anyhow::Result;

/// Main entry point for the application.
#[tokio::main]
async fn main() -> Result<()> {
    let cli_args = cli::Cli::parse();
    let server_url = get_server_url(&cli_args.host, cli_args.port);
    match cli_args.command {
        cli::Command::Get(get_args) => {
            handle_get_command(&server_url, get_args).await?;
        }
        cli::Command::Set(set_args) => {
            handle_set_command(&server_url, set_args).await?;
        }
        cli::Command::Delete(delete_args) => {
            handle_delete_command(&server_url, delete_args).await?;
        }
        cli::Command::Version => {
            commands::version::get_version()?;
        }
    }
    Ok(())
}

/// Constructs the server URL from the host and port.
fn get_server_url(host: &str, port: u16) -> String {
    format!("ws://{}:{}/jsonrpc", host, port)
}

/// Handles the get command and its subcommands.
async fn handle_get_command(server_url: &str, args: cli::GetArgs) -> Result<()> {
    match args.subcommand {
        cli::GetSubcommand::Client { client_id } => {
            commands::get::client::get_client(server_url, &client_id).await?;
        }
        
        cli::GetSubcommand::Streams => {
            commands::get::streams::get_streams(server_url).await?;
        }
        cli::GetSubcommand::Stream { stream_id } => {
            commands::get::stream::get_stream(server_url, &stream_id).await?;
        }
        cli::GetSubcommand::Groups => {
            commands::get::groups::get_groups(server_url).await?;
        }
        cli::GetSubcommand::Group { identifier } => {
            commands::get::group::get_group(server_url, &identifier).await?;
        }
        cli::GetSubcommand::Clients => {
            commands::get::clients::get_clients(server_url).await?;
        }
    }
    Ok(())
}

/// Handles the set command and its subcommands.
async fn handle_set_command(server_url: &str, args: cli::SetArgs) -> Result<()> {
    match args.subcommand {
        cli::SetSubcommand::Client { client_id, mute, volume, latency, name, group } => {
            commands::set::client::set_client(server_url, &client_id, mute, volume, latency, name, group).await?;
        }
        cli::SetSubcommand::Group { group_id, name, mute, stream_id, clients } => {
            commands::set::group::set_group(server_url, &group_id, name, mute, stream_id, clients).await?;
        }
    }
    Ok(())
}

/// Handles the delete command and its subcommands.
async fn handle_delete_command(server_url: &str, args: cli::DeleteArgs) -> Result<()> {
    match args.subcommand {
        cli::DeleteSubcommand::Client { client_id } => {
            commands::delete::client::delete_client(server_url, &client_id).await?;
        }
        cli::DeleteSubcommand::Clients { client_ids } => {
            commands::delete::clients::delete_clients(server_url, &client_ids).await?;
        }
    }
    Ok(())
}