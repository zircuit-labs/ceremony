# Zircuit Ceremony

Welcome to Zircuit Mainnet Ceremony!

The ceremony is a multi-party computation protocol designed to ensure the security of our proving system. Specifically, it aims to generate a trustable Structured Reference String (SRS) for the [KZG Polynomial Commitment](https://www.iacr.org/archive/asiacrypt2010/6477178/6477178.pdf) scheme.

During the ceremony, each participant generates a secret and uses it to re-randomize the latest available SRS. This updated SRS is then published and made available to the next participant.

When the ceremony ends, the last contribution is finalized so it can be used to compute KZG polynomial commitments in a secure and uncheatable manner. The final SRS is considered secure as long as at least one participant did not share and properly discarded their secret after contributing.

Therefore, contributions must be submitted and verified sequentially, requiring participants to book a contribution slot.

To do this, please open a new [contribution issue](https://github.com/zircuit-labs/ceremony/issues/new?assignees=&labels=contribution&projects=&template=0_contribution.md&title=New+Contribution) and provide the necessary information. 

We will notify participants in chronological order, based on the issue creation date, when it is their turn to contribute.

For more information, please refer to the [build](/docs/build.md) and [ceremony](/docs/ceremony.md#overview) instructions.

## Contributions

| Contribution | SHA256 | Issue |
|:-:|:-:|:-:|
| [`0000000000.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000000.csrs)    | `fcba4b152b3ca7af1bc82b5a3babbe0104badd134414578e7824edb12c650c12`   | [#1](https://github.com/zircuit-labs/ceremony/issues/1)   |
| [`0000000001.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000001.csrs)    | `0e2c2bf5c7157d2301aec31fbf981474b4630f6434bb147ba0c1855d632fa111`   | [#2](https://github.com/zircuit-labs/ceremony/issues/2)   |
