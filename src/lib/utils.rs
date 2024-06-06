use std::{
    collections::HashMap,
    collections::hash_map::Entry,
    fs::{self, read_dir, File},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use halo2_proofs::halo2curves::bn256::Bn256;
use log::debug;

use crate::contribution::{KZGContribution, CONTRIBUTION_EXTENSION, CONTRIBUTION_K};

pub fn read_id(filepath: &PathBuf) -> Result<u32> {
    let mut fd = File::open(filepath)?;
    let file_len = fs::metadata(filepath)?.len();

    if file_len > 8 {
        // We read the contribution's k
        let mut k = [0u8; 4];
        fd.read_exact(&mut k[..])?;
        let k: u32 = u32::from_le_bytes(k);
        if k != CONTRIBUTION_K {
            return Err(anyhow!(
                "Contribution read has k {} but should be {}",
                k,
                CONTRIBUTION_K
            ));
        }

        // We read the contribution's id
        // The id is encoded in the last 4 bytes
        fd.seek(SeekFrom::Start(file_len - 4))?;
        let mut id = [0u8; 4];
        fd.read_exact(&mut id[..])?;
        let id = u32::from_le_bytes(id);

        Ok(id)
    } else {
        Err(anyhow!("Contribution file {:#?} is malformed", filepath))
    }
}

pub fn get_contributions_list(path: &PathBuf) -> Result<HashMap<u32, PathBuf>> {
    if path.is_dir() {
        let mut contributions_list: HashMap<u32, PathBuf> = HashMap::new();

        // Iterate through the directory entries
        if let Ok(entries) = read_dir(path) {
            for entry in entries.flatten() {
                // Check if entry is a file and has the contribution extension
                let filepath = entry.path();

                if filepath.is_file()
                    && filepath.extension().and_then(std::ffi::OsStr::to_str)
                        == Some(CONTRIBUTION_EXTENSION)
                {
                    let id = read_id(&filepath)?;
                    debug!("Found contribution {:#?} with id {:#?}", filepath, id);
                    if let Entry::Vacant(vacant_entry) = contributions_list.entry(id) {
                        vacant_entry.insert(filepath);
                    } else {
                        return Err(anyhow!(format!(
                            "Two contributions have the same id {:#?}: {:#?} and {:#?}",
                            id,
                            contributions_list.get(&id).unwrap(),
                            filepath.to_owned()
                        )));
                    }
                }
            }
        }

        Ok(contributions_list)
    } else {
        Err(anyhow!("Contribution's path {:#?} is not a folder", path))
    }
}

pub fn get_last_contribution(path: &PathBuf) -> Result<KZGContribution<Bn256>> {
    let contributions_list = get_contributions_list(path)?;
    let mut contributions_ids: Vec<u32> = contributions_list.keys().cloned().collect();
    contributions_ids.sort_unstable();

    if let Some(last_contribution_id) = contributions_ids.last() {
        let last_contribution_path = contributions_list.get(last_contribution_id).unwrap();
        let last_contribution = KZGContribution::from(last_contribution_path)?;
        Ok(last_contribution)
    } else {
        Err(anyhow!("No contribution found in {:#?}", path))
    }
}
