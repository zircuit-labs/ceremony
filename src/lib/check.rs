use anyhow::{anyhow, Result};
use blake2::{Blake2b512, Digest};
use group::{cofactor::CofactorCurveAffine, ff::FromUniformBytes};
use halo2_proofs::arithmetic::Field;
use halo2_proofs::{
    arithmetic::parallelize,
    halo2curves::{
        bn256::{pairing, Bn256, Fr, G1Affine, G2Affine, G2},
        msm::best_multiexp,
        serde::SerdeObject,
    },
};
use log::{error, info, warn};
use std::path::PathBuf;

use crate::contribution::{KZGContribution, CONTRIBUTION_K, CONTRIBUTION_N};
use crate::proof::verify_contribution_proof;
use crate::utils::get_contributions_list;

pub fn check_contribution(contribution: &KZGContribution<Bn256>) -> Result<bool> {
    info!("Checking contribution with id {:#?}", contribution.id());

    let mut result: bool = true;

    let check_result = (contribution.k() == CONTRIBUTION_K) && (contribution.n() == CONTRIBUTION_N);
    if check_result {
        info!("Contribution's k and n are correct");
    } else {
        error!("Contribution's k and n are NOT correct");
    }
    result &= check_result;

    let check_result = check_srs(contribution.g(), contribution.g2(), contribution.s_g2())?;
    if check_result {
        info!("SRS is valid");
    } else {
        error!("SRS is NOT valid");
    }
    result &= check_result;

    if result {
        info!("Contribution with id {:#?} is valid", contribution.id());
    } else {
        error!("Contribution with id {:#?} is NOT valid", contribution.id());
    }

    Ok(result)
}

// It verifies that e(a^(i-1)*P, a*P') = e(a^i*P, P') for all i in [1, n]
fn check_srs(g: &Vec<G1Affine>, g2: &G2Affine, s_g2: &G2Affine) -> Result<bool> {
    let mut result: bool = true;

    // Check #1: We check that g[0] and g2 are the hardcoded generators for G1 and G2, respectively
    let check_result = (g[0] == G1Affine::generator()) && (*g2 == G2Affine::generator());
    if check_result {
        info!("> SRS Check #1 Succeded: srs uses hardcoded group generators");
    } else {
        error!("> SRS Check #1 Failed: srs does not use hardcoded group generators");
    }
    result &= check_result;

    // Check #2: We check that s_g2 has order r
    // Note that [r] * g = O <=> [r-1] * g = -g
    // Since r in Fr is encoded as 0, the latter equation allows us to actually check the order through a full-size scalar multiplication
    // Indeed, -Fr::one() is a full bitsize unsigned scalar corresponding to r-1
    let g2_proj: G2 = g2.into();
    let s_g2_proj: G2 = s_g2.into();
    let check_result =
        (g2_proj * (-Fr::one()) == -g2_proj) && (s_g2_proj * (-Fr::one()) == -s_g2_proj);
    if check_result {
        info!("> SRS Check #2 Succeded: g2 and s_g2 have order r");
    } else {
        error!("> SRS Check #2 Failed: g2 or s_g2 does not have order r");
    }
    result &= check_result;

    // Check #3 We check that g has the expected length
    let check_result = g.len() == CONTRIBUTION_N;
    if check_result {
        info!("> SRS Check #3 Succeded: the g vector has expected size");
    } else {
        error!("> SRS Check #3 Failed: the g vector has an unexpected size");
    }
    result &= check_result;

    // Check #4: We check that srs is well-formed
    // We hash all the inputs to get a random scalar rho
    let mut hasher = Blake2b512::new();
    for el in g {
        hasher.update(el.to_raw_bytes());
    }
    hasher.update(g2.to_raw_bytes());
    hasher.update(s_g2.to_raw_bytes());
    let rho = Fr::from_uniform_bytes(hasher.finalize().as_ref());

    // We create a vector rho_powers = [1, rho, rho^2, .., rho^n-1] with n = g.len()
    let mut rho_powers = vec![Fr::zero(); g.len()];
    parallelize(&mut rho_powers, |rho_powers, start| {
        let mut curr_power = rho.pow_vartime([start as u64, 0, 0, 0]);
        for rho_power in rho_powers.iter_mut() {
            *rho_power = curr_power;
            curr_power *= rho;
        }
    });

    // We compute the msm \sum_{i=0}^{n-1} rho^i * g[i]
    let g1_msm = best_multiexp(&rho_powers, g);

    // We check that e( \sum_{i=1}^{n-1} rho^{i-1} * g[i], g2 ) = e( \sum_{i=0}^{n-2} rho^i * g[i], s_g2 )
    // We compute the left pairing G1 element, i.e. \sum_{i=1}^{n-1} rho^{i-1} * g[i]
    // Note that g1_msm = \sum_{i=0}^{n-1} rho^i * g[i]
    let g1_left: G1Affine = ((g1_msm - G1Affine::generator()) * rho.invert().unwrap()).into();

    // We compute the right pairing G1 element, i.e. \sum_{i=0}^{n-2} rho^i * g[i]
    // Note that g1_msm = \sum_{i=0}^{n-1} rho^i * g[i]
    let g1_right: G1Affine = (g1_msm - g.last().unwrap() * rho_powers.last().unwrap()).into();

    let check_result = pairing(&g1_left, &G2Affine::generator()) == pairing(&g1_right, s_g2);
    if check_result {
        info!("> SRS Check #4 Succeded: srs is well-formed");
    } else {
        error!("> SRS Check #4 Failed: srs is not well-formed");
    }
    result &= check_result;

    // Check #5: srs is non-degenerative
    let check_result = g[1] != G1Affine::identity();
    if check_result {
        info!("> SRS Check #5 Succeded: srs is non-degenerative");
    } else {
        error!("> SRS Check #5 Failed: srs is degenerative");
    }
    result &= check_result;

    Ok(result)
}

pub fn check_contribution_chain(contributions_path: &str) -> Result<bool> {
    let mut chain_is_valid = true;

    let contributions_path = PathBuf::from(contributions_path);
    let contributions_list = get_contributions_list(&contributions_path)?;
    let mut contributions_ids: Vec<u32> = contributions_list.keys().cloned().collect();

    if contributions_ids.is_empty() {
        return Err(anyhow!(
            "No contribution found in {:#?}",
            contributions_path.display()
        ));
    } else {
        contributions_ids.sort_unstable();
    }

    // We iterate over all contributions found
    let mut prev_s_g: Option<G1Affine> = None;
    for id in contributions_ids.iter() {
        let curr_contribution: KZGContribution<Bn256> =
            KZGContribution::from(contributions_list.get(id).unwrap())?;

        let srs_check = check_contribution(&curr_contribution)?;

        // We verify the contribution proof.
        // Note that contribution with id 0 cannot have a contribution proof, which is set to a default value
        let proof_check = if curr_contribution.id() == 0 {
            warn!(
                "Skipping contribution proof check for contribution with id {}",
                curr_contribution.id()
            );
            true
        } else if let Some(prev_s_g) = prev_s_g {
            verify_contribution_proof(
                &prev_s_g,
                curr_contribution.s_g(),
                curr_contribution.proof(),
            )
        } else {
            error!(
                "Missing contribution with id {}: cannot verify proof for contribution with id {}",
                curr_contribution.id() - 1,
                curr_contribution.id()
            );
            false
        };
        if proof_check {
            info!(
                "Contribution proof for contribution with id {} is valid",
                curr_contribution.id()
            );
        } else {
            error!(
                "Contribution proof for contribution with id {} is NOT valid or it is NOT possible to check its validity",
                curr_contribution.id()
            );
        }

        // We save g[1] and s_g2 for next contribution, if present
        if contributions_ids.contains(&(curr_contribution.id() + 1)) {
            prev_s_g = Some(*curr_contribution.s_g());
        } else {
            prev_s_g = None;
        }

        chain_is_valid &= srs_check & proof_check;
    }

    if chain_is_valid {
        info!("Contributions' chain is valid");
    } else {
        error!("Contributions' chain is NOT valid");
    }

    Ok(chain_is_valid)
}
