use std::process::{Command, exit};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the specified command
    Run { command: String, args: Vec<String> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { command, args } => {
            let mut child = Command::new(command).args(args).spawn()?;
            let status = child.wait()?;
            if let Some(code) = status.code() {
                exit(code)
            }
        }
    }

    Ok(())
}
