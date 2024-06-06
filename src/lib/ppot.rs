use anyhow::{anyhow, Result};
use blake2::{Blake2b512, Digest};
use byteorder::{BigEndian, ReadBytesExt};
use halo2_proofs::halo2curves::bn256::{Bn256, Fq, Fq2, G1Affine, G2Affine};
use log::info;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
};

use super::contribution::KZGContribution;
use crate::{
    contribution::{CONTRIBUTION_K, CONTRIBUTION_N},
    proof::ContributionProof,
};
use std::fmt::Write;

fn read_fq(reader: &mut BufReader<File>) -> Result<Fq> {
    let mut g1_raw: [u64; 4] = [0; 4];
    for i in 0..4 {
        g1_raw[3 - i] = reader.read_u64::<BigEndian>()?;
    }
    Ok(Fq::from_raw(g1_raw))
}

fn read_g1(reader: &mut BufReader<File>) -> Result<G1Affine> {
    Ok(G1Affine {
        x: read_fq(reader)?,
        y: read_fq(reader)?,
    })
}

fn read_g2(reader: &mut BufReader<File>) -> Result<G2Affine> {
    Ok(G2Affine {
        x: Fq2 {
            c1: read_fq(reader)?,
            c0: read_fq(reader)?,
        },
        y: Fq2 {
            c1: read_fq(reader)?,
            c0: read_fq(reader)?,
        },
    })
}

// This utility reads a challenge file generated for ppot and converts it to a set of KZG Parameters encoded using halo2curves::Bn256
pub fn read_challenge(
    challenge_path: &str,
    challenge_k: u32,
    hash_challenge: bool,
) -> Result<KZGContribution<Bn256>> {
    if CONTRIBUTION_K > challenge_k {
        return Err(anyhow!("Ceremony k is {} but the input PPOT challenge has been read with k = {}. It is not possible to create a starting srs for the set ceremony k from the PPOT challenge", CONTRIBUTION_K, challenge_k));
    }

    let ppot_challenge_path = PathBuf::from(challenge_path);
    if !ppot_challenge_path.is_file() {
        return Err(anyhow!(
            "The challenge path {:#?} is not a file",
            challenge_path
        ));
    }

    let mut ppot_challenge = File::open(ppot_challenge_path)?;
    let mut reader = BufReader::new(ppot_challenge.try_clone()?);

    // If enabled, we hash the input challenge
    if hash_challenge {
        let mut hasher = Blake2b512::new();

        // We read the file in chunks of at most 1 GiB
        let mut file_buffer = vec![0u8; 1024 * 1024 * 1024];
        loop {
            let bytes_read = reader.read(&mut file_buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&file_buffer[..bytes_read]);
        }

        let mut ppot_challenge_hash: [u8; 64] = [0; 64];
        hasher.finalize_into((&mut ppot_challenge_hash).into());

        let mut hex_hash = String::with_capacity(ppot_challenge_hash.len() * 2);
        for byte in ppot_challenge_hash {
            write!(hex_hash, "{:02x}", byte)?;
        }

        info!("The PPOT Challenge hash is {:#?}", hex_hash);

        // We seek back to the beginning of file
        ppot_challenge.seek(SeekFrom::Start(0))?;
    }

    // We read the PPOT Challenge hash
    let mut ppot_challenge_hash: [u8; 64] = [0; 64];
    ppot_challenge.read_exact(&mut ppot_challenge_hash)?;

    let mut hex_hash = String::with_capacity(ppot_challenge_hash.len() * 2);
    for byte in ppot_challenge_hash {
        write!(hex_hash, "{:02x}", byte)?;
    }

    info!(
        "The reported response hash in PPOT Challenge is {:#?}",
        hex_hash
    );

    // We read the elements in G1
    let mut g: Vec<G1Affine> = Vec::with_capacity(CONTRIBUTION_N);
    for _ in 0..CONTRIBUTION_N {
        g.push(read_g1(&mut reader)?);
    }

    info!("Read {:#?} elements from TauG1", g.len());

    // Challenge's TauG1 consists of 2^{k+1}-1 elements
    let challenge_taug1_length: u64 = (1 << (challenge_k + 1)) - 1;

    // Considering the initial 64 bytes hash, TauG2 starts at byte offset 64 + 64*challenge_taug1_length
    let start: u64 = 64 + 64 * challenge_taug1_length;
    ppot_challenge.seek(SeekFrom::Start(start))?;
    let mut reader = BufReader::new(ppot_challenge.try_clone()?);

    // We read the elements in G2
    let g2 = read_g2(&mut reader)?;
    let s_g2 = read_g2(&mut reader)?;

    info!("Read 2 elements from TauG2");

    Ok(KZGContribution::<Bn256>::from_parts(
        CONTRIBUTION_K,
        g,
        g2,
        s_g2,
        ContributionProof::default(),
        0,
    ))
}
