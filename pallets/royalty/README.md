### Royalty Pallet
This pallet tries to implement a NFT model that supports [EIP-712](https://eips.ethereum.org/EIPS/eip-2981).

#### Types
For the PolkadotJs UI to fully work with this pallet you need to add the following to **Settings > Developer**.

```
{
  "Token": {
    "id": "TokenId",
    "owner": "AccountId",
    "royalty_percentage": "u16",
    "royalty_receiver": "AccountId",
    "uri": "Text"
  },
  "TokenId": "u32"
}

```

####Running Tests

For the tests to fully run, you need to to first proceed to the pallet's folder using command `cd ./pallets/royalty` from the root of this repo, and then running command `cargo test`
 *Or*

 You can run the following command:
 ```
cargo test --package pallet-royalty --lib -- tests --nocapture
 ```
