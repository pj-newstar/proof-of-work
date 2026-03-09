use clap::{Parser, Subcommand};

/// kCTF Proof-of-Work CLI tool
#[derive(Parser)]
#[command(name = "pow")]
#[command(version = "0.1.0")]
#[command(about = "kCTF Proof-of-Work tool - Rust implementation", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new challenge with specified difficulty
    #[command(aliases = ["create", "gen"])]
    Generate {
        /// Difficulty level (e.g., 1337 = 1 sec, 31337 = 30 secs, 313373 = 5 mins on 1.6GHz CPU)
        difficulty: u32,

        /// Show detailed output including challenge and difficulty
        #[arg(long)]
        detail: bool,
    },

    /// Solve a given challenge
    Solve {
        /// The challenge string to solve
        challenge: String,

        /// Show detailed output including challenge, solution, and time taken
        #[arg(long)]
        detail: bool,
    },

    /// Check if a solution is valid for a challenge
    #[command(alias = "verify")]
    Check {
        /// The challenge string
        challenge: String,

        /// The solution string to verify
        solution: String,

        /// Show detailed output including challenge, solution, and result
        #[arg(long)]
        detail: bool,

        /// Quiet mode: no output, only exit code (0=valid, 1=invalid)
        #[arg(short, long)]
        quiet: bool,
    },

    /// Start with proof-of-work challenge before executing a command
    #[command(alias = "start")]
    Run {
        /// Difficulty level (0 = disabled, e.g., 1337 = 1 sec, 31337 = 30 secs)
        difficulty: u32,

        /// Command to execute after successful verification (use -- to separate)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        exec_command: Vec<String>,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
