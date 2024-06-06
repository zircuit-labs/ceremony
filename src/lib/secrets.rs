use anyhow::Result;
use blake2::{Blake2b512, Digest};
use group::ff::FromUniformBytes;
use halo2_proofs::halo2curves::{
    bn256::{Bn256, Fr},
    pairing::Engine,
};
use log::{debug, info};
use rand::{rngs::OsRng, RngCore};
use std::io::stdin;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use zeroize::Zeroize;

/// Secret scalars that should be properly discarded
pub struct Secret<E: Engine> {
    pub value: Box<E::Fr>,
}

#[derive(Debug)]
pub struct SecretsGenConfig {
    pub files_to_hash: Option<Vec<String>>,
    pub from_stdin: bool,
    pub random_bytes_size: Option<usize>,
    pub hash_iterations: Option<u32>,
    pub reveal_s: bool,
}

impl<E: Engine> Secret<E> {
    pub fn get(&self) -> &E::Fr {
        &self.value
    }
}

impl<E: Engine> Zeroize for Secret<E> {
    fn zeroize(&mut self) {
        *self.value = E::Fr::default();
    }
}

impl<E: Engine> Drop for Secret<E> {
    fn drop(&mut self) {
        self.zeroize();
    }
}

// Implementations for SecretsGenConfig
impl Default for SecretsGenConfig {
    fn default() -> Self {
        SecretsGenConfig {
            files_to_hash: None,
            from_stdin: false,
            random_bytes_size: Some(1024 * 1024 * 1024),
            hash_iterations: Some(1 << 20),
            reveal_s: false,
        }
    }
}

pub fn gen_secrets(config: SecretsGenConfig) -> Result<(Secret<Bn256>, Secret<Bn256>)> {
    info!("Generating secrets .. ");

    let mut hasher = Blake2b512::new();

    if let Some(paths) = config.files_to_hash {
        for str_path in paths {
            let path = PathBuf::from(str_path);
            if path.is_file() {
                info!("Hashing {:#?}", path);

                let file = File::open(path)?;
                let mut reader = BufReader::new(file);

                // We read the file in chunks of at most 16 MiB
                let mut file_buffer = vec![0u8; 16 * 1024 * 1024];
                loop {
                    let bytes_read = reader.read(&mut file_buffer)?;
                    if bytes_read == 0 {
                        break;
                    }
                    hasher.update(&file_buffer[..bytes_read]);
                }
                file_buffer.zeroize();
                drop(file_buffer);
            } else {
                info!("Skipping randomness source {:#?}: not a file", &path);
            }
        }
    }

    // We read user's input from stdin
    if config.from_stdin {
        let mut stdin_buffer: Vec<u8> = Vec::new();
        let mut stdin_reader = stdin().lock();
        println!(
            "Enter any input text, and then press Ctrl+D (Unix) or Ctrl+Z (Windows) to continue."
        );
        stdin_reader.read_to_end(&mut stdin_buffer)?;

        // We hash the stdin and erase its buffer
        hasher.update(&stdin_buffer);
        stdin_buffer.zeroize();
        drop(stdin_buffer);
    }

    if let Some(random_bytes_size) = config.random_bytes_size {
        info!("Generating and hashing {} random bytes", random_bytes_size);

        // To reduce memory consumptions, we read random bytes in chunks of at most 16MiB
        let mut rng_buffer = vec![0u8; 16 * 1024 * 1024];
        for _ in 0..random_bytes_size / rng_buffer.len() {
            OsRng.fill_bytes(&mut rng_buffer);
            hasher.update(&rng_buffer);
        }
        rng_buffer.truncate(random_bytes_size % rng_buffer.len());
        OsRng.fill_bytes(&mut rng_buffer);
        hasher.update(&rng_buffer);

        rng_buffer.zeroize();
        drop(rng_buffer);
    }

    if let Some(hash_iterations) = config.hash_iterations {
        // We perform hash_iterations hashes on current hash state
        info!("Hashing hasher's state {} times", hash_iterations);
        for _ in 0..hash_iterations {
            hasher.update(hasher.clone().finalize_reset());
        }
    }

    info!("Extracting secrets");

    // We extract the secret s
    let mut s_hasher = hasher.clone();
    s_hasher.update("s");
    let mut s_bytes: [u8; 64] = s_hasher.finalize_reset().into();

    let s = Secret {
        value: Box::new(Fr::from_uniform_bytes(&s_bytes)),
    };

    s_bytes.zeroize();

    // We extract the secret z
    let z = if config.reveal_s {
        // When rescaling the srs with a public randomness source, the rescaling factor s should be publicly verifiable
        // To allow public verification of s, the blinding factor z is set to 0 during proof computation
        debug!(
            "The secret z is set to zero to enable the recomputation of the rescaling factor s."
        );
        Secret {
            value: Box::new(Fr::zero()),
        }
    } else {
        let mut z_hasher = hasher.clone();
        z_hasher.update("z");
        let mut z_bytes: [u8; 64] = z_hasher.finalize_reset().into();

        let z = Secret {
            value: Box::new(Fr::from_uniform_bytes(&z_bytes)),
        };

        z_bytes.zeroize();

        z
    };

    hasher.reset();

    Ok((s, z))
}
