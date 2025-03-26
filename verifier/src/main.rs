use clap::Parser;
use similar::TextDiff;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    commands: Vec<String>,
}

#[derive(Debug)]
struct CommandOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

fn run_command(cmd: &str) -> CommandOutput {
    let mut parts: Vec<&str> = cmd.split_whitespace().collect();
    let program = parts.remove(0);

    let output = Command::new(program)
        .args(&parts)
        .output()
        .expect("Failed to execute command");

    CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
    }
}

fn compare_outputs(expected: &CommandOutput, actual: &CommandOutput) -> bool {
    let mut has_differences = false;

    if expected.stdout != actual.stdout {
        let diff = TextDiff::from_lines(&expected.stdout, &actual.stdout);
        println!("Stdout differences:\n{}", diff.unified_diff());
        has_differences = true;
    }

    if expected.stderr != actual.stderr {
        let diff = TextDiff::from_lines(&expected.stderr, &actual.stderr);
        println!("Stderr differences:\n{}", diff.unified_diff());
        has_differences = true;
    }

    if expected.exit_code != actual.exit_code {
        println!("Exit code differences:");
        println!("Expected: {}", expected.exit_code);
        println!("Got: {}", actual.exit_code);
        has_differences = true;
    }

    !has_differences
}

fn main() {
    let args = Args::parse();

    if args.commands.is_empty() {
        eprintln!("No commands provided");
        std::process::exit(1);
    }

    println!("Checking command: {}", &args.commands[0]);
    let expected = run_command(&args.commands[0]);
    let mut all_match = true;

    for cmd in args.commands.iter().skip(1) {
        println!("Checking command: {}", cmd);
        let actual = run_command(cmd);

        if !compare_outputs(&expected, &actual) {
            all_match = false;
        }
    }

    if !all_match {
        std::process::exit(1);
    }
}
