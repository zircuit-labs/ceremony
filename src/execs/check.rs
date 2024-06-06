use anyhow::Result;
use ceremony::check::check_contribution_chain;
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[clap(author = "Zircuit Labs", version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Arguments {
    #[arg(
        short = 'c',
        long = "contributions",
        value_parser,
        required = true,
        help = "The directory containing the contributions"
    )]
    contributions_path: String,
    #[arg(short = 'H', long = "help", action = ArgAction::Help, help = "Print help information")]
    help: Option<bool>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Arguments::parse();

    assert!(check_contribution_chain(&args.contributions_path)?);

    Ok(())
}
