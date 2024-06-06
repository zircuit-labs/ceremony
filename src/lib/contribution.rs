use core::fmt::Debug;
use group::prime::PrimeCurveAffine;
use halo2_proofs::arithmetic::{g_to_lagrange, CurveExt};
use halo2_proofs::halo2curves::{pairing::Engine, serde::SerdeObject};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::ParamsKZG;
use halo2_proofs::{
    arithmetic::{parallelize, CurveAffine},
    SerdeFormat,
};
use log::info;
use std::io::{self, BufReader, BufWriter, Error, ErrorKind, Write};
use std::path::PathBuf;

use crate::contribute::default_contribution_filepath;
use crate::proof::ContributionProof;

pub const CONTRIBUTION_K: u32 = 28;
pub const CONTRIBUTION_N: usize = 1 << CONTRIBUTION_K;
pub const CONTRIBUTION_EXTENSION: &str = "csrs";

/// KZG public parameters for ceremony contributors
#[derive(Debug, Clone)]
pub struct KZGContribution<E: Engine> {
    k: u32,
    n: usize,
    g: Vec<E::G1Affine>,
    g2: E::G2Affine,
    s_g2: E::G2Affine,
    proof: ContributionProof<E>,
    id: u32,
}

impl<E: Engine + Debug> Default for KZGContribution<E> {
    fn default() -> Self {
        Self {
            k: CONTRIBUTION_K,
            n: CONTRIBUTION_N,
            g: vec![<E::G1Affine as PrimeCurveAffine>::generator(); CONTRIBUTION_N],
            g2: <E::G2Affine as PrimeCurveAffine>::generator(),
            s_g2: <E::G2Affine as PrimeCurveAffine>::generator(),
            proof: ContributionProof::<E>::default(),
            id: 0,
        }
    }
}

impl<E: Engine + Debug> KZGContribution<E>
where
    E::G1Affine: SerdeCurveAffine,
{
    pub fn from_parts(
        k: u32,
        g: Vec<E::G1Affine>,
        g2: E::G2Affine,
        s_g2: E::G2Affine,
        proof: ContributionProof<E>,
        id: u32,
    ) -> Self {
        KZGContribution {
            k,
            n: 1 << k,
            g,
            g2,
            s_g2,
            proof,
            id,
        }
    }

    /// Returns the contribution's k
    pub fn k(&self) -> u32 {
        self.k
    }

    /// Returns n = 1 << k
    pub fn n(&self) -> usize {
        self.n
    }

    /// Returns the contribution's G1Affine Elements
    pub fn g(&self) -> &Vec<E::G1Affine> {
        &self.g
    }

    /// Returns the generator of G2
    pub fn g2(&self) -> &E::G2Affine {
        &self.g2
    }

    /// Returns first power of secret on G1
    pub fn s_g(&self) -> &E::G1Affine {
        &self.g[1]
    }

    /// Returns first power of secret on G2
    pub fn s_g2(&self) -> &E::G2Affine {
        &self.s_g2
    }

    /// Returns the Contribution id
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the Contribution proof
    pub fn proof(&self) -> &ContributionProof<E> {
        &self.proof
    }

    pub fn to_params(self) -> ParamsKZG<E>
    where
        E::G1Affine: SerdeCurveAffine<ScalarExt = <E as Engine>::Fr, CurveExt = <E as Engine>::G1>,
        E::G1: CurveExt<AffineExt = E::G1Affine>,
        E::G2Affine: SerdeCurveAffine,
    {
        info!("Converting contribution with id {} to ParamsKZG", self.id());

        info!(
            "Computing Lagrange basis for contribution with id {}",
            self.id()
        );
        let g_lagrange: Vec<E::G1Affine> = g_to_lagrange(
            self.g.iter().map(PrimeCurveAffine::to_curve).collect(),
            self.k,
        );
        info!("Lagrange basis computed..");

        // ParamsKZG::from_parts takes &self as input. We create an empty struct (k=0) and populate it with the contribution field
        let empty_params = ParamsKZG::new(0);

        empty_params.from_parts(self.k, self.g, Some(g_lagrange), self.g2, self.s_g2)
    }

    /// Writes a contribution to disk
    pub fn write(&self, filepath: &PathBuf) -> io::Result<()>
    where
        E::G2Affine: SerdeCurveAffine,
        <E as Engine>::Fr: SerdeObject,
    {
        let fd_write = std::fs::File::create(filepath)?;
        let mut buffered_writer = BufWriter::new(fd_write);
        self.write_custom(&mut buffered_writer, SerdeFormat::RawBytes)?;
        buffered_writer.flush()
    }

    /// Writes a contribution to disk to a default path generated from the contribution's id
    pub fn write_default(&self, contributions_path_str: &str) -> io::Result<()>
    where
        E::G2Affine: SerdeCurveAffine,
        <E as Engine>::Fr: SerdeObject,
    {
        let contributions_path = PathBuf::from(contributions_path_str);
        let filepath = match default_contribution_filepath(&contributions_path, self.id()) {
            Ok(path) => Ok(path),
            Err(e) => Err(io::Error::new(ErrorKind::Other, e.to_string())),
        }?;

        let fd_write = std::fs::File::create(&filepath)?;
        let mut buffered_writer = BufWriter::new(fd_write);
        self.write_custom(&mut buffered_writer, SerdeFormat::RawBytes)?;
        buffered_writer.flush()?;

        info!("Contribution written to {:#?}", &filepath.display());

        Ok(())
    }

    /// Reads a contribution from disk
    pub fn from(filepath: &PathBuf) -> io::Result<Self>
    where
        E::G2Affine: SerdeCurveAffine,
        <E as Engine>::Fr: SerdeObject,
    {
        info!("Reading contribution from {:#?}", &filepath.display());
        let fd_read = std::fs::File::open(filepath)?;
        let mut buffered_reader = BufReader::new(fd_read);
        KZGContribution::read_custom(&mut buffered_reader, SerdeFormat::RawBytes)
    }

    /// Writes parameters to buffer
    pub fn write_custom<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()>
    where
        E::G2Affine: SerdeCurveAffine,
        <E as Engine>::Fr: SerdeObject,
    {
        writer.write_all(&self.k.to_le_bytes())?;
        for el in self.g.iter() {
            el.write(writer, format)?;
        }
        self.g2.write(writer, format)?;
        self.s_g2.write(writer, format)?;
        self.proof.write(writer)?;
        writer.write_all(&self.id.to_le_bytes())?;
        Ok(())
    }

    /// Reads params from a buffer.
    pub fn read_custom<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self>
    where
        E::G2Affine: SerdeCurveAffine,
        <E as Engine>::Fr: SerdeObject,
    {
        let mut k = [0u8; 4];
        reader.read_exact(&mut k[..])?;
        let k = u32::from_le_bytes(k);
        if k != CONTRIBUTION_K {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Contribution read has k {} but should be {}",
                    k, CONTRIBUTION_K
                ),
            ));
        }
        let n = 1 << k;

        let g = match format {
            SerdeFormat::Processed => {
                use group::GroupEncoding;
                let load_points_from_file_parallelly =
                    |reader: &mut R| -> io::Result<Vec<Option<E::G1Affine>>> {
                        let mut points_compressed =
                            vec![<<E as Engine>::G1Affine as GroupEncoding>::Repr::default(); n];
                        for point_compressed in points_compressed.iter_mut() {
                            reader.read_exact((*point_compressed).as_mut())?;
                        }

                        let mut points = vec![Option::<E::G1Affine>::None; n];
                        parallelize(&mut points, |points, chunks| {
                            for (i, point) in points.iter_mut().enumerate() {
                                *point = Option::from(E::G1Affine::from_bytes(
                                    &points_compressed[chunks + i],
                                ));
                            }
                        });
                        Ok(points)
                    };

                let g = load_points_from_file_parallelly(reader)?;
                let g: Vec<<E as Engine>::G1Affine> = g
                    .iter()
                    .map(|point| {
                        point.ok_or_else(|| {
                            io::Error::new(io::ErrorKind::Other, "invalid point encoding")
                        })
                    })
                    .collect::<Result<_, _>>()?;
                g
            }
            SerdeFormat::RawBytes => (0..n)
                .map(|_| <E::G1Affine as SerdeCurveAffine>::read(reader, format))
                .collect::<Result<Vec<_>, _>>()?,
            SerdeFormat::RawBytesUnchecked => {
                // avoid try branching for performance
                (0..n)
                    .map(|_| <E::G1Affine as SerdeCurveAffine>::read(reader, format).unwrap())
                    .collect::<Vec<_>>()
            }
        };

        let g2 = <E::G2Affine as SerdeCurveAffine>::read(reader, format)?;
        let s_g2 = <E::G2Affine as SerdeCurveAffine>::read(reader, format)?;

        let proof = ContributionProof::read(reader)?;

        let mut id = [0u8; 4];
        reader.read_exact(&mut id[..])?;
        let id = u32::from_le_bytes(id);

        // We ensure all points read are on the curve
        if !g.iter().all(|&point| point.is_on_curve().into()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Contribution contains one or more points in G1 not on the curve",
            ));
        }

        if !Into::<bool>::into(Into::<E::G1Affine>::into(*proof.p()).is_on_curve()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Contribution proof's point p is not on the curve",
            ));
        }

        if !Into::<bool>::into(g2.is_on_curve()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Contribution has point g2 not on the curve",
            ));
        }

        if !Into::<bool>::into(s_g2.is_on_curve()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Contribution has point s_g2 not on the curve",
            ));
        }

        Ok(Self {
            k,
            n,
            g,
            g2,
            s_g2,
            proof,
            id,
        })
    }
}

pub trait SerdeCurveAffine: CurveAffine + SerdeObject {
    /// Reads an element from the buffer and parses it according to the `format`:
    /// - `Processed`: Reads a compressed curve element and decompress it
    /// - `RawBytes`: Reads an uncompressed curve element with coordinates in Montgomery form.
    /// Checks that field elements are less than modulus, and then checks that the point is on the curve.
    /// - `RawBytesUnchecked`: Reads an uncompressed curve element with coordinates in Montgomery form;
    /// does not perform any checks
    fn read<R: io::Read>(reader: &mut R, format: SerdeFormat) -> io::Result<Self> {
        match format {
            SerdeFormat::Processed => <Self as CurveRead>::read(reader),
            SerdeFormat::RawBytes => <Self as SerdeObject>::read_raw(reader),
            SerdeFormat::RawBytesUnchecked => Ok(<Self as SerdeObject>::read_raw_unchecked(reader)),
        }
    }
    /// Writes a curve element according to `format`:
    /// - `Processed`: Writes a compressed curve element
    /// - Otherwise: Writes an uncompressed curve element with coordinates in Montgomery form
    fn write<W: io::Write>(&self, writer: &mut W, format: SerdeFormat) -> io::Result<()> {
        match format {
            SerdeFormat::Processed => writer.write_all(self.to_bytes().as_ref()),
            _ => self.write_raw(writer),
        }
    }

    /// Byte length of an affine curve element according to `format`.
    fn byte_length(format: SerdeFormat) -> usize {
        match format {
            SerdeFormat::Processed => Self::default().to_bytes().as_ref().len(),
            _ => Self::Repr::default().as_ref().len() * 2,
        }
    }
}
impl<C: CurveAffine + SerdeObject> SerdeCurveAffine for C {}

// Keep this trait for compatibility with IPA serialization
pub(crate) trait CurveRead: CurveAffine {
    /// Reads a compressed element from the buffer and attempts to parse it
    /// using `from_bytes`.
    fn read<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut compressed = Self::Repr::default();
        reader.read_exact(compressed.as_mut())?;
        Option::from(Self::from_bytes(&compressed))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid point encoding in proof"))
    }
}
impl<C: CurveAffine> CurveRead for C {}

impl<E: Engine + Debug> PartialEq for KZGContribution<E>
where
    <E as Engine>::Fr: SerdeObject,
    <E as Engine>::G1Affine: SerdeCurveAffine,
{
    fn eq(&self, other: &Self) -> bool {
        (self.k == other.k)
            && (self.n == other.n)
            && (self.g.len() == other.g.len())
            && (self.g == other.g)
            && (self.g2 == other.g2)
            && (self.s_g2 == other.s_g2)
            && (self.proof.p() == other.proof.p())
            && (self.proof.r() == other.proof.r())
            && (self.id == other.id)
    }
}
