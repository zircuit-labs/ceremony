use anyhow::Result;
use ceremony::{contribute::contribute, secrets::SecretsGenConfig};
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[clap(author = "Zircuit Labs", version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Arguments {
    #[arg(
        short = 'c',
        long,
        value_parser,
        required = true,
        help = "The directory containing the contributions"
    )]
    contributions_path: String,
    #[arg(
        short = 'f',
        long = "files_to_hash",
        value_parser,
        help = "Hash the provided file into the hash state"
    )]
    files_to_hash: Option<Vec<String>>,
    #[arg(
        short = 'i',
        long = "stdin",
        help = "Hash input from stdin into the hash state"
    )]
    from_stdin: bool,
    #[arg(
        short = 'r',
        long = "random_size",
        value_parser,
        help = "Hash the specified number of random bytes into the hash state"
    )]
    random_bytes_size: Option<usize>,
    #[arg(
        short = 'h',
        long = "hash_iterations",
        value_parser,
        help = "Hash the hash state for the specified number of iterations"
    )]
    hash_iterations: Option<u32>,
    #[arg(
        short = 'p',
        long = "public",
        help = "Reveal the secret used for contribution"
    )]
    reveal_s: bool,
    #[arg(short = 'H', long = "help", action = ArgAction::Help, help = "Print help information")]
    help: Option<bool>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Arguments::parse();

    let config = if args.files_to_hash.is_some()
        || args.from_stdin
        || args.random_bytes_size.is_some()
        || args.hash_iterations.is_some()
        || args.reveal_s
    {
        SecretsGenConfig {
            files_to_hash: args.files_to_hash,
            from_stdin: args.from_stdin,
            random_bytes_size: args.random_bytes_size,
            hash_iterations: args.hash_iterations,
            reveal_s: args.reveal_s,
        }
    } else {
        SecretsGenConfig::default()
    };

    contribute(&args.contributions_path, config)
}
