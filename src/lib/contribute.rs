use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use group::{ff::Field, prime::PrimeCurveAffine};
use halo2_proofs::{
    arithmetic::parallelize,
    halo2curves::bn256::{Bn256, Fr, G1Affine, G2Affine},
};
use log::info;

use crate::{
    check::check_contribution,
    contribution::{KZGContribution, CONTRIBUTION_EXTENSION, CONTRIBUTION_K},
    proof::create_contribution_proof,
    secrets::{gen_secrets, Secret, SecretsGenConfig},
    utils::get_last_contribution,
};

// This utility is used to generate a default filename for storing new contributions
pub fn default_contribution_filepath(
    contributions_path: &Path,
    contribution_id: u32,
) -> Result<PathBuf> {
    let mut path = contributions_path.to_path_buf();

    if path.is_dir() {
        let filename = format!("{:010}", contribution_id);

        path.push(filename);
        path.set_extension(CONTRIBUTION_EXTENSION);

        if path.exists() {
            return Err(anyhow!(
                "The generated default filepath {:#?} points to an already existing file",
                path.display()
            ));
        }

        Ok(path)
    } else {
        Err(anyhow!(
            "Contributions path {:#?} is not a directory",
            path.display()
        ))
    }
}

pub fn contribute(contributions_path_str: &str, config: SecretsGenConfig) -> Result<()> {
    let contributions_path = PathBuf::from(contributions_path_str);

    let last_contribution = get_last_contribution(&contributions_path)?;

    let is_valid = check_contribution(&last_contribution)?;
    if !is_valid {
        return Err(anyhow!(
            "The retrieved last contribution with id {:#?} is not valid",
            last_contribution.id()
        ));
    }

    // We generate 2 secrets (s for rescaling the srs, z for the contribution proof)
    // according to the config
    let (s, z) = gen_secrets(config)?;

    let contribution = rescale(last_contribution, &s, &z);

    // We explicitly drop the secrets
    drop(s);
    drop(z);

    // We store the contribution in contributions_path with a default filename
    contribution.write_default(contributions_path_str)?;

    Ok(())
}

pub fn rescale(
    prev_contribution: KZGContribution<Bn256>,
    s: &Secret<Bn256>,
    z: &Secret<Bn256>,
) -> KZGContribution<Bn256> {
    info!("Rescaling powers of tau..");

    // Compared to setup + batch_normalize, this approach seems to use less memory but has comparable performances
    let mut g = vec![G1Affine::identity(); 1 << CONTRIBUTION_K];
    parallelize(&mut g, |chunk_g, start| {
        let mut _s_pow = s.get().pow_vartime([start as u64, 0, 0, 0]);
        for (idx, g_el) in chunk_g.iter_mut().enumerate() {
            *g_el = (prev_contribution.g()[start + idx] * _s_pow).into();
            _s_pow *= s.get();
        }
        // We zero the _s_pow value before being dropped
        _s_pow = Fr::zero();
    });

    let s_g2: G2Affine = (prev_contribution.s_g2() * s.get()).into();

    // We compute a contribution proof
    info!("Computing contribution proof..");
    let contribution_proof = create_contribution_proof(prev_contribution.s_g(), s, z);

    KZGContribution::from_parts(
        CONTRIBUTION_K,
        g,
        G2Affine::generator(),
        s_g2,
        contribution_proof,
        prev_contribution.id() + 1,
    )
}
