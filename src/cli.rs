use clap::{Parser, Subcommand};

/// Command-line interface for the application.
/// Main CLI structure for the Snapcast Control Utility.
#[derive(Parser, Debug)]
#[clap(author, version, about = "Snapcast Control Utility", long_about = None)]
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

/// Commands available in the application.
/// Enum representing the available commands.
#[derive(Subcommand, Debug)]
pub enum Command {
    Get(GetArgs),
    Set(SetArgs),
    Delete(DeleteArgs),
    Version,
}


/// Arguments for the 'get' command.
#[derive(Parser, Debug)]
pub struct GetArgs {
    /// Subcommand for the get command.
    #[clap(subcommand)]
    pub subcommand: GetSubcommand,
}


/// Enum representing the available subcommands for the 'get' command.
#[derive(Subcommand, Debug)]
pub enum GetSubcommand {
    
    Streams,
    Stream { stream_id: String },
    Groups,
    Group { identifier: String },
    Clients,
    Client { client_id: String },
}

/// Arguments for the 'set' command.
#[derive(Parser, Debug)]
pub struct SetArgs {
    /// Subcommand for the set command.
    #[clap(subcommand)]
    pub subcommand: SetSubcommand,
}

/// Enum representing the available subcommands for the 'set' command.
#[derive(Subcommand, Debug)]
pub enum SetSubcommand {

    Client {
        client_id: String,

        #[clap(long)]
        mute: Option<bool>,

        #[clap(long)]
        volume: Option<i64>,

        #[clap(long)]
        latency: Option<i64>,

        #[clap(long)]
        name: Option<String>,

        #[clap(long)]
        group: Option<String>,
    },

    Group {

        group_id: String,

        #[clap(long)]
        name: Option<String>,

        #[clap(long)]
        mute: Option<bool>,

        #[clap(long)]
        stream_id: Option<String>,

        #[clap(long)]
        clients: Option<String>,
    },
}


/// Arguments for the 'delete' command.
#[derive(Parser, Debug)]
pub struct DeleteArgs {
    /// Subcommand for the delete command.
    #[clap(subcommand)]
    pub subcommand: DeleteSubcommand,
}


/// Enum representing the available subcommands for the 'delete' command.
#[derive(Subcommand, Debug)]
pub enum DeleteSubcommand {
    Client { client_id: String },
    Clients { client_ids: String },
}
