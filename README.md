## Running tests

1. `cd contracts/scores`
2. `cargo test`

## Bonus task 1

The basic version is in commit b55bf45, while HEAD has the version with the bonus task. I simply changed the type of the
key in the state to `(Addr, String)` to complete the bonus. I wasn't sure what type would be relevant for the name of
the token, however, the task didn't specify any validation requirements, so I didn't delve into any of that.

## Bonus task 2 (record of testnet session)

```
$ terrain deploy scores --signer bombay --network testnet

...

$ terrain console --network testnet

terrain > await lib.getOwner()
{ owner: 'terra1yd89rj39rxhxyzq8gt4j03yveylxq3rn8uprva' }

terrain > await lib.getScore("alice", "terra")
{ unscored: {} }

terrain > await lib.setScore(wallets.bombay2, "alice", "terra", 10)
Uncaught Error: Request failed with status code 400
  [...]
      message: 'failed to execute message; message index: 0: Unauthorized: execute wasm contract failed: invalid request',
  [...]

terrain > await lib.setScore(wallets.bombay, "alice", "terra", 10)
  txhash: '3AFC2E1FE0C7BF024E3CBA95C75918FDC2CCCBDC4169CD7572119D00913221E8',
  [...]

terrain > await lib.getScore("alice", "terra")
{ score: { addr: 'alice', token: 'terra', score: 10 } }

terrain > await lib.getScore("alice", "luna")
{ unscored: {} }

terrain > await lib.getScore("bob", "luna")
{ unscored: {} }

terrain > await lib.setScore(wallets.bombay, "alice", "terra", 20)
  txhash: 'F6877B59E714A7B9B91FB69A14794B6E6B19FB577829E73972D1F07D28804A64',
  [...]

terrain > await lib.getScore("alice", "terra")
{ score: { addr: 'alice', token: 'terra', score: 20 } }
```
