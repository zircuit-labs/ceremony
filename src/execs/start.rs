use anyhow::Result;
use ceremony::{contribution::KZGContribution, ppot::read_challenge};
use clap::{ArgAction, Parser};
use halo2_proofs::halo2curves::bn256::Bn256;
use log::info;

#[derive(Parser)]
#[clap(author = "Zircuit Labs", version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Arguments {
    #[arg(
        short = 'c',
        long = "contributions",
        value_parser,
        required = true,
        help = "The directory for storing the initial contribution"
    )]
    contributions_path: String,
    #[arg(
        short = 'p',
        long = "ppot",
        value_parser,
        requires = "challenge_k",
        help = "The file path for the PPoT challenge"
    )]
    challenge_path: Option<String>,
    #[arg(
        short = 'k',
        long = "ppot_k",
        value_parser,
        requires = "challenge_path",
        help = "The k value used to compute the PPoT challenge"
    )]
    challenge_k: Option<u32>,
    #[arg(
        short = 'h',
        long = "hash",
        value_parser,
        requires = "challenge_path",
        help = "Hash the PPoT challenge file for verification purposes"
    )]
    hash_challenge: bool,
    #[arg(short = 'H', long = "help", action = ArgAction::Help, help = "Print help information")]
    help: Option<bool>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Arguments::parse();

    let contribution = if let Some(challenge_path) = &args.challenge_path {
        let challenge_k = args.challenge_k.unwrap();
        info!("Reading PPOT Challenge {:#?}", challenge_path);
        read_challenge(challenge_path, challenge_k, args.hash_challenge)?
    } else {
        info!("Creating a default SRS");
        KZGContribution::<Bn256>::default()
    };

    contribution.write_default(&args.contributions_path)?;

    Ok(())
}
