# Zircuit Ceremony

Welcome to Zircuit Mainnet Ceremony!

![ceremony](https://camo.githubusercontent.com/aece4fbcf5ca7eb74bceadad7449ad9e812f5d5e2c994dc3af5cec31ac1f2294/68747470733a2f2f63646e2e70726f642e776562736974652d66696c65732e636f6d2f3635326463373730313639343233653136616334613764612f3636363231616664353232306666636566646336323061665f6b7a67253230636572656d6f6e792e706e67)

**The ceremony ended on June 22, 2024. Thanks to all contributors and to everyone who expressed an interest in participating! Please check [below](#parameters) for the finalized parameters.**

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
| [`0000000000.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000000.csrs)    | `fcba4b152b3ca7af1bc82b5a3babbe0104badd134414578e7824edb12c650c12`   | [#1](https://github.com/zircuit-labs/ceremony/issues/1)   |
| [`0000000001.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000001.csrs)    | `0e2c2bf5c7157d2301aec31fbf981474b4630f6434bb147ba0c1855d632fa111`   | [#2](https://github.com/zircuit-labs/ceremony/issues/2)   |
| [`0000000002.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000002.csrs)    | `4f3065f68d7e4291050ecc6e6323cba8c1705a30f320fbbfcece3d1cb1f85dca`   | [#9](https://github.com/zircuit-labs/ceremony/issues/9)   |
| [`0000000003.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000003.csrs)    | `2107971af18d3cc75a8b68dc9d526ce27921595dba0c6bdcbeda4975cfb63f8b`   | [#4](https://github.com/zircuit-labs/ceremony/issues/4)   |
| [`0000000004.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000004.csrs)    | `46b7ec969f2cbfffac33fa93d427c63b44152d28bc25452ff21e71b99a5e30fa`   | [#5](https://github.com/zircuit-labs/ceremony/issues/5)   |
| [`0000000005.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000005.csrs)    | `f64efe664bc653435ea02d51dd46a11a4e78f20f66c1ef972a6cc457f8da96c2`   | [#6](https://github.com/zircuit-labs/ceremony/issues/6)   |
| [`0000000006.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000006.csrs)    | `c702a8c1a762b948c02293aa10b35f5d2b42cf116e7da119c378d45dd0b7d507`   | [#15](https://github.com/zircuit-labs/ceremony/issues/15)   |
| [`0000000007.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000007.csrs)    | `a39d9c7ed21fa100ba8a8e71d3f8150e73ec50746ae2177ed6472cba190333c7`   | [#7](https://github.com/zircuit-labs/ceremony/issues/7)   |
| [`0000000008.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000008.csrs)    | `d04398dfb06dfd6e1689f4472fd98e9dea0408761ae01bb339091e4e69947881`   | [#14](https://github.com/zircuit-labs/ceremony/issues/14)   |
| [`0000000009.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000009.csrs)    | `d8893b11610daba1f00f6974098d88b3100d469220bdeed9fbf057dde90c42c8`   | [#16](https://github.com/zircuit-labs/ceremony/issues/16)   |
| [`0000000010.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000010.csrs)    | `6d614f7b992ab5d58e42dca6ad290bbfd1fa64b602cbc3a648a82c202cfbf73f`   | [#11](https://github.com/zircuit-labs/ceremony/issues/11)   |
| [`0000000011.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000011.csrs)    | `51d0b51fd89b404621d339aad9424db8037a1367e71e8c49bdcd0e827d475f33`   | [#17](https://github.com/zircuit-labs/ceremony/issues/17)   |
| [`0000000012.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000012.csrs)    | `2a5ba4c2a301357f9729775632d98d8d4d2deabcd79f5fea8bff8e6584e9f630`   | [#18](https://github.com/zircuit-labs/ceremony/issues/18)   |
| [`0000000013.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000013.csrs)    | `fe694fe59369a729ae30f1e372f4914ebce71b7d59bd530fd52e66e9a8da1108`   | [#19](https://github.com/zircuit-labs/ceremony/issues/19)   |
| [`0000000014.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000014.csrs)    | `138557a82019c4ac80c676dd29e4a96bce6c7789d196d4177a46794db5d9ff46`   | [#23](https://github.com/zircuit-labs/ceremony/issues/23)   |
| [`0000000015.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000015.csrs)    | `5492ade6e7803c7cb15acab18f2a28468be3b6408b6489edce193023c6253381`   | [#25](https://github.com/zircuit-labs/ceremony/issues/25)   |
| [`0000000016.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000016.csrs)    | `edfdfd3401f387a64b5fd16eda1712ff6ed6948f70cd6909b9210b4cb7d66b24`   | [#29](https://github.com/zircuit-labs/ceremony/issues/29)   |
| [`0000000017.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000017.csrs)    | `7897c4b235febd1a410b3549de54c4f695eb3d75f44aa0705a09c37e8ea81c45`   | [#30](https://github.com/zircuit-labs/ceremony/issues/30)   |
| [`0000000018.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000018.csrs)    | `15f930f5f447eb8f81c95020db2150bbb1ade24f3c791d9ca5daa9569ecc11aa`   | [#98](https://github.com/zircuit-labs/ceremony/issues/98)   |
| [`0000000019.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000019.csrs)    | `a4f35fb5fba41433e76a65dffb4bbb3295c7785abb77ad2c465aef364f3b4ee8`   | [#33](https://github.com/zircuit-labs/ceremony/issues/33)   |
| [`0000000020.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000020.csrs)    | `f1616034820cd7a393b79533e1e92adbed583ad67ae3042e80a59253288d1865`   | [#21](https://github.com/zircuit-labs/ceremony/issues/21)   |
| [`0000000021.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000021.csrs)    | `8f91776118e07bcf4d201f6b24171b6413d92f2fc07ad4b8e007a98a399e03a8`   | [#99](https://github.com/zircuit-labs/ceremony/issues/99)   |
| [`0000000022.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000022.csrs)    | `b55cc8d621ef7d4435b5d863f672ed9b52dc677ad96fa5f253f13067d3ccb654`   | [#110](https://github.com/zircuit-labs/ceremony/issues/110)   |
| [`0000000023.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000023.csrs)    | `310f6d1cc5a730199b8de9df2436a9ec99a735a2e273d507648d329de84c2796`   | [#118](https://github.com/zircuit-labs/ceremony/issues/118)   |
| [`0000000024.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000024.csrs)    | `eeb480b59df5b5c0e8ef97fad8c922eac7777c9c7c647ff6eeac913bde6ff68f`   | [#96](https://github.com/zircuit-labs/ceremony/issues/96)   |
| [`0000000025.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000025.csrs)    | `7038adeec1d753331eb017331e92701cb25b34a84410ee32d9a969a3b10c5cb3`   | [#43](https://github.com/zircuit-labs/ceremony/issues/43)   |
| [`0000000026.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000026.csrs)    | `e8472290215b42574696987812f121ed562a3ec13af974d9610e7b0628414a73`   | [#124](https://github.com/zircuit-labs/ceremony/issues/124)   |
| [`0000000027.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000027.csrs)    | `8fe261137bc810792a9018956a7dd365d1704138235bc612753b104312ea79d2`   | [#105](https://github.com/zircuit-labs/ceremony/issues/105)   |
| [`0000000028.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000028.csrs)    | `78c2651b6cf2773680f32fdb935ebb77b47107bbed889d99273a1ff96b167bfb`   | [#22](https://github.com/zircuit-labs/ceremony/issues/22)   |
| [`0000000029.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000029.csrs)    | `e46dd9e7d39be42a4733314469262a98bae4987957065ccbc2edfe5a3ed6c27f`   | [#120](https://github.com/zircuit-labs/ceremony/issues/120)   |
| [`0000000030.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000030.csrs)    | `7c8cfc3fd2006af57db36b1a87b2a29743ae6fe6b193b2253157a7ca1ed2359e`   | [#31](https://github.com/zircuit-labs/ceremony/issues/31)   |
| [`0000000031.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000031.csrs)    | `8edb74d271409a436cc9ffddc1b8a6c6fae8d5be2523bf9da3e0489a6d2e853c`   | [#122](https://github.com/zircuit-labs/ceremony/issues/122)   |
| [`0000000032.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000032.csrs)    | `e07b64cf7800edbd9208f69d1da37fd85fc5b2afbae986d2b4e129862c57d7f9`   | [#139](https://github.com/zircuit-labs/ceremony/issues/139)   |
| [`0000000033.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000033.csrs)    | `45a7a887bc5645e42c7119bf9668d5ec25e4e29f93ef006d3e781855ae194058`   | [#141](https://github.com/zircuit-labs/ceremony/issues/141)   |
| [`0000000034.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000034.csrs)    | `b20f065ff0e514e0385e2aa792ab9ae6b3068d5608949fa298873c20519f5f6c`   | [#140](https://github.com/zircuit-labs/ceremony/issues/140)   |
| [`0000000035.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000035.csrs)    | `52f753bdbe5ae731227baa93b11941805c4db9fb825cb4f16dc7c36410c18180`   | [#145](https://github.com/zircuit-labs/ceremony/issues/145)   |
| [`0000000036.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000036.csrs)    | `61a4513b5895a1a15189b29904fcd46fd53bd3c2ec6df45f4452c843b68635ab`   | [#149](https://github.com/zircuit-labs/ceremony/issues/149)   |
| [`0000000037.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000037.csrs)    | `e0cc1a849172df6ec7eeba5e9a5137be7f010616881747c88a2197d88ed0e587`   | [#156](https://github.com/zircuit-labs/ceremony/issues/156)   |
| [`0000000038.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000038.csrs)    | `58fb2c9fdc1f970429eb174ad4bb59cc30775d716330583ea6c76459dd4aa38d`   | [#128](https://github.com/zircuit-labs/ceremony/issues/128)   |
| [`0000000039.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000039.csrs)    | `801f913ac08cb30d8b405d131866d7a384555f241075ea2df09b0a67b9883e4d`   | [#12](https://github.com/zircuit-labs/ceremony/issues/12)   |
| [`0000000040.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000040.csrs)    | `b1a16960a1ceab5754eb7e1c9cabdbcc0a825a6a292c8b45116a6e15a811da03`   | [#68](https://github.com/zircuit-labs/ceremony/issues/68)   |
| [`0000000041.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000041.csrs)    | `be97e46dc0b782e681dc1fdb6f8ebaadbaab8a5ac90003a6c034a426d7363d45`   | [#142](https://github.com/zircuit-labs/ceremony/issues/142)   |
| [`0000000042.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000042.csrs)    | `80f7a9bbb15c035b91a487166caecb16a43699bef4104ce1695d4992bb93278b`   | [#179](https://github.com/zircuit-labs/ceremony/issues/179)   |
| [`0000000043.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000043.csrs)    | `5713f1f727d9b46e2ed782151dd033a3721e4657ea6fcc006e8ac389aaf351bc`   | [#190](https://github.com/zircuit-labs/ceremony/issues/190)   |
| [`0000000044.csrs`](https://mainnet-ceremony.s3.amazonaws.com/contributions/0000000044.csrs)    | `2fd6229e2abeb34197781213975739a994b82e9495ab27f4a28352d1b5b4d534`   | [#50](https://github.com/zircuit-labs/ceremony/issues/50)   |

## Parameters

The following parameters were obtained by [finalizing](/docs/ceremony.md#finalize) the latest contribution and [downsizing](https://github.com/privacy-scaling-explorations/halo2/blob/360020745ee68447af82ec4427ba1434d9b3d23f/halo2_backend/src/poly/kzg/commitment.rs#L291-L299) the result to generate parameters files for all `k` values from `1` to `28`. See [here](https://github.com/zircuit-labs/ceremony/issues/774) for more information.

| `k` | SHA256 |
|:-:|:-:|
| [`k_28.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_28.srs) | `5ad5d1c974b58215f2e3ecdd15c354dfe49cacc2edf118ecc27a6f91067e0651` |
| [`k_27.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_27.srs) | `a86cf1de983e136780b1b219e68d10c17bbab5b4ffba123075800a82c30133ee` |
| [`k_26.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_26.srs) | `1a311585bf8f9f6c3876a393fa0b05d8c214a3092b49e3b16b95f020164d086c` |
| [`k_25.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_25.srs) | `4fe16a44dc119745971a87df0320db3332fdfbd22da7a966e8bbe34fd9fb88d5` |
| [`k_24.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_24.srs) | `06b77e49b7a9ab8c01acff10ea2bb468bafafafca128dabf59fd178e3089eff3` |
| [`k_23.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_23.srs) | `93b8e9602532f361345a403e0f33318096f0334fc83565b0099740b155b31992` |
| [`k_22.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_22.srs) | `5568e9ec4031f41cbddc786a23c9ece05d693d5771f15a4feb82f9ff35548dca` |
| [`k_21.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_21.srs) | `84efe3df1ae26cc0f617b8363fdc5ab6e612e3641abf8c642cb0a7377ad9d8ec` |
| [`k_20.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_20.srs) | `2700c632a3b4c598f84752fd370155708c66425ca5ea9c1d52d001f3926bbcf8` |
| [`k_19.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_19.srs) | `ad51681156aca16803d6686d96251016f82817b6467c4ea6fb27b6ddcd9ab75d` |
| [`k_18.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_18.srs) | `50fe86556336671c815813f65dce3c827263b6ca854a4ef63c3c8e26b93f4737` |
| [`k_17.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_17.srs) | `bd19355c0c055ae0c7cb29a3a3317accf29003249474449e9cec0ab4678b54ce` |
| [`k_16.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_16.srs) | `f6cadb0c76589d8aedf54f5e93d9041d3e369826d0e80f6e0cdb30276a7e8d9e` |
| [`k_15.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_15.srs) | `f13d2d43590c6fcf4d9ff4150178052c4284eded548586af0e45f2df767d856e` |
| [`k_14.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_14.srs) | `e8342590b140499b75cbfa393f85e7924445f4b4cdeefae3718c91b311f3c4ae` |
| [`k_13.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_13.srs) | `1149d7789131ce37615fec286a93b7002c4faecd64198d10ce60c921b17d60f1` |
| [`k_12.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_12.srs) | `30315c7590b5576a43ef7149862e8367d207b812296d9bb302d89e9dd51c0eaa` |
| [`k_11.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_11.srs) | `780852da11b17f14303bad944cbb569244444cac4969ad5351c76b971968fdff` |
| [`k_10.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_10.srs) | `c5d75f14e9fa1a2912dde846ae3f85ae5d12c882632f29f36bfc1bf049d2bcc4` |
|  [`k_9.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_9.srs) | `b2e30ea231625a1bd26fd2e7676c0249d82ab4016296d07e96f243915a4d1e37` |
|  [`k_8.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_8.srs) | `77528837befc0cea186b04ae6ebc40ae8061a7237f251c4b2cab0548ca13f1ae` |
|  [`k_7.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_7.srs) | `78c6387afff3f891e114019e8c887304606804a47b1530e7e7cf7f03134149c9` |
|  [`k_6.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_6.srs) | `25025195c0dec8e7dc3ec1b62aba26fcb2b7398bc5af0aeae9e1dc3cc0f1eafb` |
|  [`k_5.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_5.srs) | `9286e7b670880c097cd7f1c8fe2bdad98c785614d4df14d7020997e1cf699767` |
|  [`k_4.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_4.srs) | `e58d443dafc4bc7760265fdcf5ec50f7ff92929e0e954a56228d3dcc0752d1d9` |
|  [`k_3.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_3.srs) | `ecfa1aed31d24b9e0d2ac2dc0c705e30a20308b0427822079de1dd787d251c04` |
|  [`k_2.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_2.srs) | `8f54fb121e8070b6bdd8d7e6c7bda8ee3c5681a0313a7cac7cfc345b9063b5ba` |
|  [`k_1.srs`](https://mainnet-ceremony.s3.eu-north-1.amazonaws.com/parameters/k_1.srs) | `0f87793b076dc420d8584eb1d104686f179823f113bfb85cdafd928e5deaac61` |