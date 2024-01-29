use clap::Parser;

mod args;
mod config;
mod providers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Hello, world!{}",
        confy::get_configuration_file_path(config::APP_NAME, None)
            .unwrap()
            .to_str()
            .unwrap()
    );
    config::set_cfg(confy::load(config::APP_NAME, None).unwrap());

    config::set_provider(config::get_cfg().provider.create_provider().await);

    let args = args::Args::parse();

    match args.subcommand {
        Some(args::Commands::Config) => {
            // Handle "config" subcommand
        }
        Some(args::Commands::Migrate { region }) => {
            let region = match region {
                Some(region) => region,
                None => config::get_provider().find_optimal_region().await,
            };
            config::get_provider().migrate_region(region).await;
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
    Ok(())
}
