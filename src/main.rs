use clap::Parser;

mod args;
mod config;
mod providers;

fn main() {
    let args = args::Args::parse();

    match args.subcommand {
        Some(args::Commands::Config) => {
            // Handle "config" subcommand
        }
        Some(args::Commands::Migrate { region }) => {
            // Handle "migrate" subcommand
        }
        Some(args::Commands::Workon { ctf }) => {
            // Handle "workon" subcommand
        }
        Some(args::Commands::Share { dir }) => {
            // Handle "share" subcommand
        }
        Some(args::Commands::Up) => {
            // Handle "up" subcommand
        }
        Some(args::Commands::Down) => {
            // Handle "down" subcommand
        }
        None => {
            // Handle no subcommand case
        }
    }
}
