use crate::utils::get_last_contribution;
use anyhow::{anyhow, Result};
use halo2_proofs::poly::commitment::Params;
use log::{info, warn};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

const KZG_PARAMS_FILENAME: &str = "final";
const KZG_PARAMS_EXTENSION: &str = "params";

pub fn finalize(contributions_path: String, params_filepath: Option<String>) -> Result<()> {
    let contributions_path = PathBuf::from(&contributions_path);

    // We get the filepath where ParamsKZG will be written
    let params_filepath = if let Some(path) = params_filepath {
        PathBuf::from(path)
    } else {
        let mut default_path = contributions_path.clone();
        default_path.push(format!("{}.{}", KZG_PARAMS_FILENAME, KZG_PARAMS_EXTENSION));
        warn!("No custom filepath set for output parameters.");
        default_path
    };

    // If the file already exists we return without any further expensive computation
    if params_filepath.is_file() {
        return Err(anyhow!(
            "ParamsKZG will be written to {}, but this file already exists.",
            &params_filepath.display()
        ));
    }

    // We retrieve the last contribution from contributions_path
    let last_contribution = get_last_contribution(&contributions_path)?;

    // We convert the last contribution to halo2 ParamsKZG
    let params = last_contribution.to_params();

    // We write the ParamsKZG to disk
    let fd_write = std::fs::File::create(&params_filepath)?;
    let mut buffered_writer = BufWriter::new(fd_write);
    params.write(&mut buffered_writer)?;
    buffered_writer.flush()?;

    info!(
        "ParamsKZG with k = {} written to {:#?}",
        params.k(),
        &params_filepath.display()
    );

    Ok(())
}
