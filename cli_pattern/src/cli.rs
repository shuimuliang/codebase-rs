use std::process::ExitCode;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
// use console::style;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("daemon is offline")]
    DaemonUnavailable,
    #[error("{}", .0.message())]
    Grpc(#[from] tonic::Status),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    InvalidArgument(String),
}

#[async_trait]
pub trait RunCommand {
    async fn run(self) -> Result<ExitCode, CliError>;
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

// use crate::commands::{
//     connect::Connect, disconnect::Disconnect, error::CliError, locations::ListLocations,
//     sign_in::SignIn, sign_out::SignOut, status::Status,
// };

use clap::Args;
#[derive(Args, Debug)]
pub struct SignIn {
    email: Option<String>,
}
#[derive(Args, Debug)]
pub struct SignOut;
#[derive(Args, Debug)]
pub struct Status;
#[derive(Args, Debug)]
pub struct ListLocations;
#[derive(Args, Debug)]
pub struct Connect;
#[derive(Args, Debug)]
pub struct Disconnect;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sign in to your <https://upvpn.app> account
    SignIn(SignIn),
    /// Sign out current device
    SignOut(SignOut),
    /// Current VPN status
    Status(Status),
    /// Available locations for VPN
    Locations(ListLocations),
    /// Connect VPN
    Connect(Connect),
    /// Disconnect VPN
    Disconnect(Disconnect),
}

impl Cli {
    pub async fn run(self) -> ExitCode {
        match self.command {
            Commands::SignIn(_sign_in) => (),
            Commands::SignOut(_sign_out) => (),
            Commands::Locations(_list_locations) => (),
            Commands::Connect(_connect) => (),
            Commands::Disconnect(_disconnect) => (),
            Commands::Status(_status) => (),
        };

        ExitCode::SUCCESS
        // match output {
        //     Ok(()) => ExitCode::SUCCESS,
        //     Err(e) => {
        //         eprintln!("{}", style(e).for_stderr().red());
        //         ExitCode::FAILURE
        //     }
        // }
    }
}
