# The Ceremony


## Overview

The goal of the ceremony is to compute a *Structured Reference Strings* (SRS) consisting of a set of elliptic curves points $\{P, \tau \cdot P, \tau^2 \cdot P, \dots, \tau^{2^{k}-1} \cdot P, Q, \tau \cdot Q\}$ where  $\tau$ is an unknown scalar and $P$ and $Q$ are generators of the subgroups $\mathbb{G}_1$ and $\mathbb{G}_2$, respectively, of the BN256 elliptic curve.

The ceremony consists of 3 main stages, each managed by a different binary having the same name:

- `start`: the first contribution is generated. This is either a trivial SRS or a contribution derived from a [Perpetual Powers of Tau](https://github.com/privacy-scaling-explorations/perpetualpowersoftau) challenge file.

- `contribute`: the actual contribution phase. One after each other, users produce a new contribution from the latest available by using some secret information only in their possession.

- `finalize`: the last contribution is finalized, converted to halo2 [ParamsKZG]() and stored to disk. Such parameters are those that will be employed to prove circuits.

At any time, contributions can be verified through the `check` binary, which performs consistency checks on available contributions.

## Start

This stage is performed only once by the Ceremony's administrators and its goal is to provide a starting SRS to users on top of which new contributions can be computed.

The initial contribution can be obtained either by generating a trivial SRS or by extracting the relevant values from a challenge to the [Perpetual Powers of Tau](https://github.com/privacy-scaling-explorations/perpetualpowersoftau) ceremony.


### From a trivial SRS

To start the ceremony with a trivial SRS, run:

```shell
mkdir -p contributions
start -c ./contributions 
```

This creates in `./contributions` an initial contribution consisting of $2^{28}$ and $2$ copies of the generator of $\mathbb{G}_1$ and $\mathbb{G}_2$, respectively. In other words, this is a SRS with $\tau = 1$.

### From a Perpertual Powers of Tau Challenge

In order to start the ceremony from a contribution to the [Perpetual Powers of Tau Ceremony](https://github.com/privacy-scaling-explorations/perpetualpowersoftau), we need the corresponding challenge file and the $k$ value for which such challenge has been computed.

For example, we can download the following challenge file generated for $k = 28$:
```shell
curl -O https://pse-trusted-setup-ppot.s3.eu-central-1.amazonaws.com/challenge_0086
```

and then create the initial contribution to the ceremony by running

```shell
mkdir -p contributions
start -c ./contributions -p challenge_0086 -k 28
```

An initial contribution created from a Perpetual Powers of Tau challenge file containes the first $2^{28}$ and $2$ elements of the challenge's [tau_powers_g1](https://github.com/kobigurk/phase2-bn254/blob/dd6b96657d16c1a2b81fd23e540581c356284ec6/powersoftau/src/batched_accumulator.rs#L36) and [tau_powers_g2](https://github.com/kobigurk/phase2-bn254/blob/dd6b96657d16c1a2b81fd23e540581c356284ec6/powersoftau/src/batched_accumulator.rs#L38), respectively.

If the optional option `-h` is passed, the challenge file will be hashed and the resulting hash will be printed as `log::info`.

### Supported options

The binary `start` supports the following options:

```
Usage: start [OPTIONS] --contributions <CONTRIBUTIONS_PATH>

Options:
  -c, --contributions <CONTRIBUTIONS_PATH>  The directory for storing the initial contribution
  -p, --ppot <CHALLENGE_PATH>               The file path for the PPoT challenge
  -k, --ppot_k <CHALLENGE_K>                The k value used to compute the PPoT challenge
  -h, --hash                                Hash the PPoT challenge file for verification purposes
  -H, --help                                Print help information
  -V, --version                             Print version
```

## Contribute

To participate in the ceremony, a local copy of the most recent contribution is required:

```shell
chmod +x ./download.sh
./download.sh
```

The `download.sh` bash script will automatically download the latest contribution in `./contributions/`. Downloads will automatically resume if they were previously interrupted.

To generate a contribution using the default settings:

```shell
contribute -c ./contributions/
```

This generates a new contribution by rerandomizing the latest SRS with a secret scalar $s$ calculated by hashing 1GiB of random bytes sourced from `OsRng` and then repeatedly hashing the resulting value $2^{20}$ times.

In formulas, if the latest SRS is $$SRS_{i-1} = \{P, \tau_{i-1} \cdot P, \tau_{i-1}^2 \cdot P, \dots, \tau_{i-1}^{2^{k}-1} \cdot P, \,\, Q, \tau_{i-1} \cdot Q\}$$ then the $i$-th user will rerandomize it with their secret scalar $s$ as $$SRS_{i} = \{P, (s\cdot\tau_{i-1}) \cdot P, (s^2\cdot\tau_{i-1}^2) \cdot P, \dots, (s^{2^{k}-1}\cdot\tau_{i-1}^{2^{k}-1}) \cdot P, \,\, Q, (s\cdot\tau_{i-1}) \cdot Q\}$$

The way such secret $s$ is derived can be customised by enabling the following options:

- `-f`: hashes the provided file into the hash state;
- `-i`: hashes input from stdin into the hash state;
- `-r`: hashes the specified number of random bytes sourced from `OsRng` into the hash state;
- `-h`: hashes the current hash state for the specified number of iterations.

So, for example, running

```shell
contribute -c ./contributions/ -f file1 -f file2 -i -r 1024 -h 2048
```
will generats a new contribution by rerandomizing the latest SRS in `./contributions/` with a secret scalar obtained by hashing
- the files `file1` and `file2`
- the input provided to `stdin`
- $1024$ bytes sourced by `OsRng`

and, in turn, iteratively hashing the obtained hash for $2048$ times.

Default settings are overridden once any of the `-f`, `-i`, `-r`, `-h`, or `-p` options are set.

### Publicly Verifiable Source of Randomness

To disclose the value used for rerandomizing the SRS when generating a contribution, simply pass the `-p` flag:

```shell
contribute -c contributions -r 1024 -p
```

This allows, for example, to publicly verify if a certain random beacon (passed as file or stdin) has been used for computing the scalar that generated a certain contribution.

In [this](#check) section we detail how to verify if such a scalar has been disclosed during a contribution.

### Preventing Sleep Mode During Contribution

Since computing a contribution can take several hours to complete (approximately 1 hour and 15 minutes on an Apple Macbook Pro M2 with 32GiB of RAM), we strongly recommend adjusting your computer’s power settings if you don't plan to actively use your machine during this period. 

This is because some architectures significantly reduce the performance of background processes when the machine is locked or goes into sleep mode to conserve energy. Therefore, locking your computer during the contribution phase can substantially increase the total time required to compute a contribution.

To prevent your computer from sleeping or entering a low-power state:

- On Windows, go to Settings > System > Power & sleep and set both the screen and sleep settings to 'Never'.
- On macOS, go to System Preferences > Energy Saver and adjust the settings to prevent your Mac from sleeping.
- On Linux, you can adjust power settings using your desktop environment's power management tool (e.g., GNOME Power Manager, KDE Power Management).

These adjustments will ensure your computations run at full capacity without interruptions.
 
### Supported options


The binary `contribute` supports the following options:
```
Usage: contribute [OPTIONS] --contributions-path <CONTRIBUTIONS_PATH>

Options:
  -c, --contributions-path <CONTRIBUTIONS_PATH>
          The directory containing the contributions
  -f, --files_to_hash <FILES_TO_HASH>
          Hash the provided file into the hash state
  -i, --stdin
          Hash input from stdin into the hash state
  -r, --random_size <RANDOM_BYTES_SIZE>
          Hash the specified number of random bytes into the hash state
  -h, --hash_iterations <HASH_ITERATIONS>
          Hash the hash state for the specified number of iterations
  -p, --public
          Reveal the secret used for contribution
  -h, --help
          Print help
  -V, --version
          Print version
```

## Check

Once the contribution phase concludes, it is essential to verify not only the validity of each individual contribution but also the integrity of the entire chain of contributions.

We consider a contribution valid if it meets various constraints, including: 
- use hardcoded group generators;
- have expected size;
- being well-formed;
- being non-degenerative.

The validity of the whole contribution chain, instead, is ensured by iteratively verifying the (zero-knowledge) proofs of knowledge embedded in each contribution.

Such a proof of knowledge confirms that the contributor created their contribution based on the latest available one and that they know the secret used to rerandomize it.

This proof is always zero-knowledge unless a contribution is produced using a [publicly verifiable source of randomness](#publicly-verifiable-source-of-randomness): in this case, the proof reveals the scalar used for SRS rerandomization to public scrutiny, while maintaining the validity of the chain.

The checks and the Proof of Knowledge protocol implemented in the `check` binary are detailed in the "Powers-of-tau setup with full data on-chain" section of the paper ["Powers-of-Tau to the People:
Decentralizing Setup Ceremonies"](https://eprint.iacr.org/2022/1592.pdf) by Nikolaenko, Ragsdale, Bonneau and Boneh.


To download all contributions:

```shell
chmod +x ./download.sh
./download.sh all
```
The `download.sh` bash script will automatically download all contributions in `./contributions/`. Downloads will automatically resume if they were previously interrupted.

To check all contributions in a folder:

```shell
check -c ./contributions
```

### Supported Options

The binary `check` supports the following options:

```
Usage: check --contributions <CONTRIBUTIONS_PATH>

Options:
  -c, --contributions <CONTRIBUTIONS_PATH>  The directory containing the contributions
  -H, --help                                Print help information
  -V, --version                             Print version
```

## Finalize

Once the contribution phase is concluded, and after verifying that the whole contributions' chain is valid, the last contribution is finalized and converted to a parameters file encoding a [ParamsKZG struct](https://github.com/privacy-scaling-explorations/halo2/blob/360020745ee68447af82ec4427ba1434d9b3d23f/halo2_backend/src/poly/kzg/commitment.rs#L21-L28), which, in turn, can be used in [halo2](https://github.com/privacy-scaling-explorations/halo2) to prove circuits employing the [KZG Polynomial Commitment Scheme](https://www.iacr.org/archive/asiacrypt2010/6477178/6477178.pdf).

The `finalize` binary will first check the validity of the last contributions found, compute a Lagrange Basis for the points read, and then save the result disk as `ParamsKZG`:

```shell
finalize -c ./contributions
```

By default, the finalized parameters will be written to the file `final.params` inside the specified contributions folder. To change it:

```shell
finalize -c ./contributions -o /path/parameters.srs
```

### Supported options

The binary `finalize` supports the following options:

```
Usage: finalize [OPTIONS] --contributions <CONTRIBUTIONS_PATH>

Options:
  -c, --contributions <CONTRIBUTIONS_PATH>  The directory containing the contributions
  -o, --output <PARAMS_FILEPATH>            The output filepath for the finalized parameters
  -H, --help                                Print help information
  -V, --version                             Print version
```
