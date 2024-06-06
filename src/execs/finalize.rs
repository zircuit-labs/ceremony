use anyhow::Result;
use ceremony::finalize::finalize;
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
    #[arg(
        short = 'o',
        long = "output",
        value_parser,
        help = "The output filepath for the finalized parameters"
    )]
    params_filepath: Option<String>,
    #[arg(short = 'H', long = "help", action = ArgAction::Help, help = "Print help information")]
    help: Option<bool>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Arguments::parse();

    finalize(args.contributions_path, args.params_filepath)
}
