use clap::{Parser, Subcommand, Args};


#[derive(Parser)]
#[command(author, version, about = "A tiny container runtime")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a container
    Create(CreateArgs),
    /// Execute the user defined process in a created container
    Start(StartArgs),
    /// Send the specified signal (default: SIGTERM) to the container's init process
    Kill(KillArgs),
    /// Delete a container
    Delete(DeleteArgs),
    /// Output the state of a container
    State(StateArgs),
}

#[derive(Args)]
struct CreateArgs {
    #[arg(short = 'b', long = "bundle", help = "path to the root of the bundle directory, defaults to the current directory")]
    bundle: Option<String>,

    #[arg(long = "console-socket", help = "path to an AF_UNIX socket which will receive a file descriptor referencing the master end of the console's pseudoterminal")]
    console_socket: Option<String>,

    #[arg(long = "pid-file", help = "specify the file to write the process id to")]
    pid_file: Option<String>,
}

#[derive(Args)]
struct StartArgs {
    #[arg(value_name = "container-id")]
    container_id: String,
}

#[derive(Args)]
struct KillArgs {
    #[arg(value_name = "container-id")]
    container_id: String,
    #[arg(value_name = "signal")]
    signal: Option<String>,
}

#[derive(Args)]
struct DeleteArgs {
    #[arg(value_name = "container-id")]
    container_id: String,
}

#[derive(Args)]
struct StateArgs {
    #[arg(value_name = "container-id")]
    container_id: String,
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(_) => {
            println!("created");
        }
        Commands::Start(_) => {
            println!("started");
        }
        Commands::Kill(_) => {
            println!("killed");
        }
        Commands::Delete(_) => {
            println!("started");
        }
        Commands::State(_) => {
            println!("started");
        }
    }
}
