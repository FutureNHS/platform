# Platform

## Recommended tools for developers

NOTE: You can find information on setting up the cluster and relevent tooling [here](https://github.com/FutureNHS/futurenhs-platform/blob/master/infrastructure/README.md).

### Connecting to databases in your cluster

[tableplus](https://www.tableplus.io/download) might be useful.

### Linting

The following are used within CI to verify formatting. You may find it useful to use locally too.

- [shfmt](https://github.com/mvdan/sh)
- [rustfmt](https://github.com/rust-lang/rustfmt)
- [prettier](https://prettier.io)
- [eslint](https://eslint.org/)

Note: Prettier formats all relevent code in this repo, and therefore is installed at the top level.
We also use husky for our precommit hooks. Run `yarn` in the top level to install these tools.
