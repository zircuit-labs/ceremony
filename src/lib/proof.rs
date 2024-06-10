use crate::contribution::SerdeCurveAffine;
use crate::secrets::Secret;
use blake2::{Blake2b512, Digest};
use group::ff::{Field, FromUniformBytes};
use group::prime::PrimeCurveAffine;
use halo2_proofs::halo2curves::bn256::{Bn256, Fr, G1Affine};
use halo2_proofs::halo2curves::{pairing::Engine, serde::SerdeObject};
use halo2_proofs::SerdeFormat;
use log::{error, info};
use std::fmt::Debug;
use std::io;

/// KZG public parameters for ceremony contributors
#[derive(Debug, Clone)]
pub struct ContributionProof<E: Engine> {
    p: E::G1Affine,
    r: E::Fr,
}

impl<E: Engine + Debug> Default for ContributionProof<E> {
    fn default() -> Self {
        Self {
            p: <E::G1Affine as PrimeCurveAffine>::identity(),
            r: <E::Fr as Field>::ZERO,
        }
    }
}

/// ContributionProof implementations
impl<E: Engine + Debug> ContributionProof<E>
where
    E::G1Affine: SerdeCurveAffine,
    E::Fr: SerdeObject,
{
    // Computes a zero knowledge proof that the contributor knows secret
    pub fn new(p: E::G1Affine, r: E::Fr) -> Self {
        ContributionProof { p, r }
    }

    pub fn p(&self) -> &E::G1Affine {
        &self.p
    }

    pub fn r(&self) -> &E::Fr {
        &self.r
    }

    /// Writes contribution proof to buffer
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.p.write(writer, SerdeFormat::RawBytes)?;
        self.r.write_raw(writer)?;
        Ok(())
    }

    /// Reads contribution proof from a buffer.
    pub fn read<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let p = <E::G1Affine as SerdeCurveAffine>::read(reader, SerdeFormat::RawBytes)?;
        let r = <E::Fr as SerdeObject>::read_raw(reader)?;
        Ok(ContributionProof { p, r })
    }
}

// Creates a zero-knowledge proof that the contributor knows the secret employed during the contribution phase
pub fn create_contribution_proof(
    g: &G1Affine,
    s: &Secret<Bn256>,
    z: &Secret<Bn256>,
) -> ContributionProof<Bn256> {
    let s_g: G1Affine = (g * s.get()).into();
    let p: G1Affine = (g * z.get()).into();

    let mut hasher = Blake2b512::new();
    hasher.update(s_g.to_raw_bytes());
    hasher.update(g.to_raw_bytes());
    hasher.update(p.to_raw_bytes());
    let h = Fr::from_uniform_bytes(hasher.finalize().as_ref());

    let r = z.get() + h * s.get();

    ContributionProof::new(p, r)
}

// Verifies a contribution proof
pub fn verify_contribution_proof(
    prev_s_g: &G1Affine,
    s_g: &G1Affine,
    proof: &ContributionProof<Bn256>,
) -> bool {
    if prev_s_g == s_g {
        error!("The previous s_g is equal to the current s_g. The contribution proof is invalid, or the SRS was not correctly re-randomized.");
        return false;
    }

    let mut hasher = Blake2b512::new();
    hasher.update(s_g.to_raw_bytes());
    hasher.update(prev_s_g.to_raw_bytes());
    hasher.update(proof.p().to_raw_bytes());
    let h: Fr = Fr::from_uniform_bytes(hasher.finalize().as_ref());

    let check_result = prev_s_g * proof.r() == proof.p() + s_g * h;

    // When rescaling the srs with a public randomness source, the rescaling factor s should be publicly verifiable
    // To allow public verification of s, the blinding factor z is set to 0 during proof computation
    // We check if the proof is valid and if it has been computed using z = 0 (i.e., p = G1::identity()), and, if so, we recompute s
    if check_result && proof.p().is_identity().into() {
        let s = h.invert().unwrap() * proof.r();
        info!("Contribution generated using a public randomness source. The rescaling factor s is {:#?}", s);
    }

    check_result
}
