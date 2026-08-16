use std::ffi::CString;
use std::process::exit;

use clap::{Parser, Subcommand};
use nix::errno::Errno;
use nix::sys::signal::Signal;
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{execvp, sethostname};

use nix::sched::{CloneFlags, clone};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the specified command
    Run {
        command: CString,
        args: Vec<CString>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { command, mut args } => {
            args.insert(0, command);
            let mut stack = [0; 65536];

            unsafe {
                let pid = clone(
                    Box::new(|| {
                        sethostname("container").unwrap();
                        let _ = execvp(&args[0], &args);

                        Errno::last_raw() as _
                    }),
                    &mut stack,
                    CloneFlags::CLONE_NEWUTS,
                    Some(Signal::SIGCHLD as _),
                )?;

                match waitpid(pid, None)? {
                    WaitStatus::Exited(_, code) => exit(code),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
