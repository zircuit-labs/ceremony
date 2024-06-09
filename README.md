# Zircuit Ceremony

Welcome to Zircuit Mainnet Ceremony!

![ceremony](https://camo.githubusercontent.com/aece4fbcf5ca7eb74bceadad7449ad9e812f5d5e2c994dc3af5cec31ac1f2294/68747470733a2f2f63646e2e70726f642e776562736974652d66696c65732e636f6d2f3635326463373730313639343233653136616334613764612f3636363231616664353232306666636566646336323061665f6b7a67253230636572656d6f6e792e706e67)

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
| [`0000000002.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000002.csrs)    | `4f3065f68d7e4291050ecc6e6323cba8c1705a30f320fbbfcece3d1cb1f85dca`   | [#9](https://github.com/zircuit-labs/ceremony/issues/9)   |
| [`0000000003.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000003.csrs)    | `2107971af18d3cc75a8b68dc9d526ce27921595dba0c6bdcbeda4975cfb63f8b`   | [#4](https://github.com/zircuit-labs/ceremony/issues/4)   |
| [`0000000004.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000004.csrs)    | `46b7ec969f2cbfffac33fa93d427c63b44152d28bc25452ff21e71b99a5e30fa`   | [#5](https://github.com/zircuit-labs/ceremony/issues/5)   |
| [`0000000005.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000005.csrs)    | `f64efe664bc653435ea02d51dd46a11a4e78f20f66c1ef972a6cc457f8da96c2`   | [#6](https://github.com/zircuit-labs/ceremony/issues/6)   |
| [`0000000006.csrs`](https://mainnet-ceremony.s3.amazonaws.com/valid/0000000006.csrs)    | `c702a8c1a762b948c02293aa10b35f5d2b42cf116e7da119c378d45dd0b7d507`   | [#15](https://github.com/zircuit-labs/ceremony/issues/15)   |
