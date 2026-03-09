mod cli;
mod crypto;

use anyhow::Result;
use cli::{Cli, Commands};
use colored::Colorize;
use crypto::Pow;
use std::io::{self, Write};
use std::process::Command;
use std::time::Instant;

fn main() {
    let cli = Cli::parse_args();

    let result = match cli.command {
        Commands::Generate { difficulty, detail } => handle_generate(difficulty, detail),
        Commands::Solve { challenge, detail } => handle_solve(&challenge, detail),
        Commands::Check {
            challenge,
            solution,
            detail,
            quiet,
        } => handle_check(&challenge, &solution, detail, quiet),
        Commands::Run {
            difficulty,
            exec_command,
        } => handle_run(difficulty, exec_command),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e.to_string());
        std::process::exit(1);
    }
}

fn handle_generate(difficulty: u32, detail: bool) -> Result<()> {
    // Check if difficulty is 0
    if difficulty == 0 {
        anyhow::bail!("Difficulty must be greater than 0");
    }

    // Generate and output the challenge
    let challenge = Pow::generate(difficulty)?;

    if detail {
        println!("{} {}", "Challenge:".cyan().bold(), challenge);
        println!(
            "{} {}",
            "Difficulty:".cyan().bold(),
            difficulty.to_string().yellow()
        );
    } else {
        println!("{}", challenge);
    }

    Ok(())
}

fn handle_solve(challenge: &str, detail: bool) -> Result<()> {
    let start = Instant::now();

    // Solve the challenge
    let solution = Pow::solve(challenge)?;

    let elapsed = start.elapsed();

    // Verify the solution
    let is_valid = Pow::verify(challenge, &solution)?;

    if !is_valid {
        anyhow::bail!("Generated solution is invalid");
    }

    if detail {
        println!("{} {}", "Challenge:".cyan().bold(), challenge);
        println!("{} {}", "Solution:".magenta().bold(), solution);
        println!(
            "{} {}",
            "Taken:".cyan().bold(),
            format!("{} ms", elapsed.as_millis()).blue()
        );
    } else {
        println!("{}", solution);
    }

    Ok(())
}

fn handle_check(challenge: &str, solution: &str, detail: bool, quiet: bool) -> Result<()> {
    // Verify the solution
    let is_valid = Pow::verify(challenge, solution)?;

    // Quiet mode has highest priority
    if quiet {
        // No output, only exit code
        if !is_valid {
            std::process::exit(1);
        }
        return Ok(());
    }

    if detail {
        println!("{} {}", "Challenge:".cyan().bold(), challenge);
        println!("{} {}", "Solution:".cyan().bold(), solution);
        let valid_str = if is_valid {
            "true".green()
        } else {
            "false".red()
        };
        println!("{} {}", "Valid:".magenta().bold(), valid_str);
    } else {
        println!("{}", is_valid);
    }

    if !is_valid {
        std::process::exit(1);
    }

    Ok(())
}

fn handle_run(difficulty: u32, exec_command: Vec<String>) -> Result<()> {
    if difficulty == 0 {
        // Proof of work disabled
        println!("== Proof of work: disabled ==");
        io::stdout().flush()?;
        let exit_code = execute_command_if_present(exec_command)?;
        std::process::exit(exit_code);
    }

    // Generate challenge
    let challenge = Pow::generate(difficulty)?;

    // get from env SOLVER_URL or default to goo.gle/kctf-pow
    let solver_url = std::env::var("SOLVER_URL").unwrap_or_else(|_| "goo.gle/kctf-pow".to_string());

    // Display prompt
    println!("== Proof of work: enabled ==");
    println!("Please solve a PoW first.");
    println!("Challenge: {}", challenge);
    println!("You can run the solver with:");
    println!("  python3 <(curl -sSL {solver_url}) solve {challenge}");
    print!("Solution: ");
    io::stdout().flush()?;

    // Read solution from stdin
    let mut solution = String::new();
    io::stdin().read_line(&mut solution)?;
    let solution = solution.trim();

    // Verify the solution
    match Pow::verify(&challenge, solution) {
        Ok(true) => {
            println!("Proof of work correct!");
            println!("============================");
            io::stdout().flush()?;
            let exit_code = execute_command_if_present(exec_command)?;
            std::process::exit(exit_code);
        }
        Ok(false) => {
            println!("Proof of work verification failed.");
            println!("============================");
            io::stdout().flush()?;
            std::process::exit(1);
        }
        Err(_) => {
            println!("Proof of work verification failed.");
            println!("============================");
            io::stdout().flush()?;
            std::process::exit(1);
        }
    }
    // Ok(())
}

fn execute_command_if_present(exec_command: Vec<String>) -> Result<i32> {
    if exec_command.is_empty() {
        return Ok(0);
    }

    // Execute the command
    let status = if exec_command.len() == 1 {
        Command::new(&exec_command[0]).status()?
    } else {
        Command::new(&exec_command[0])
            .args(&exec_command[1..])
            .status()?
    };

    io::stdout().flush()?;
    io::stderr().flush()?;
    // return the exit code of the command
    Ok(status.code().unwrap_or(1))
}
