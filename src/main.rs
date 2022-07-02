use clap::{Parser, Subcommand};
use std::error::Error;

mod client;
use client::Client;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check your balance.
    Balance,
    /// Send money to a user.
    Send {
        /// The recipient's Discord ID.
        to: String,
        /// The amount of NotNetCoin to send.
        amount: u32,
    },
    /// Claim your daily NotNetCoin.
    Daily,
    /// Log in to NotNetCoin.
    Login,
}

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let mut client = Client::new();

    match cli.command {
        Commands::Balance => client.check_balance()?,
        Commands::Send { to, amount } => client.send_money(to, amount)?,
        Commands::Daily => client.claim_daily()?,
        Commands::Login => {
            let url = "https://discord.com/api/oauth2/authorize?client_id=991697126784520293&redirect_uri=https%3A%2F%2Fnnc.n2.pm%2Foauth&response_type=code&scope=identify";
            println!(
                "Opening in your browser. If it doesn't work, use this URL: {}",
                url
            );
            open::that(url)?;
        }
    }

    Ok(())
}
