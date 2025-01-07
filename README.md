# Enhanced [create2crunch](https://github.com/0age/create2crunch) with CLI Arguments and Custom Pattern Features

This fork extends the original CREATE2 address generator with additional capabilities:

## New Features

- Command Line Interface (CLI) using Clap framework
- Support for custom prefix/suffix Ethereum address pattern finding

## CLI Arguments

The tool now accepts command line arguments for:

- Specifying custom prefix/suffix patterns for Ethereum addresses

## Address Pattern Features

- Ability to find Ethereum addresses matching specific prefix patterns
- Support for suffix-based filtering
- Enhanced pattern matching capabilities for CREATE2 address generation

## Improvements

- Command line interface for better usability
- Extended functionality for specific Ethereum address pattern requirements

For detailed usage instructions and examples, see the CLI help documentation(`cargo run -- --help`).

# create2crunch

> A Rust program for finding salts that create gas-efficient Ethereum addresses via CREATE2.

Provide three arguments: a factory address (or contract that will call CREATE2), a caller address (for factory addresses that require it as a protection against frontrunning), and the keccak-256 hash of the initialization code of the contract that the factory will deploy.
(The example below references the `Create2Factory`'s address on one of the 21 chains where it has been deployed to.)

Live `Create2Factory` contracts can be found [here](https://blockscan.com/address/0x0000000000ffe8b47b3e2130213b802212439497).

```sh
$ git clone https://github.com/0age/create2crunch
$ cd create2crunch
$ export FACTORY="0x0000000000ffe8b47b3e2130213b802212439497"
$ export CALLER="<YOUR_DEPLOYER_ADDRESS_OF_CHOICE_GOES_HERE>"
$ export INIT_CODE_HASH="<HASH_OF_YOUR_CONTRACT_INIT_CODE_GOES_HERE>"
$ cargo run --release $FACTORY $CALLER $INIT_CODE_HASH
```

For each efficient address found, the salt, resultant addresses, and value _(i.e. approximate rarity)_ will be written to `efficient_addresses.txt`. Verify that one of the salts actually results in the intended address before getting in too deep - ideally, the CREATE2 factory will have a view method for checking what address you'll get for submitting a particular salt. Be sure not to change the factory address or the init code without first removing any existing data to prevent the two salt types from becoming commingled. There's also a _very_ simple monitoring tool available if you run `$python3 analysis.py` in another tab.

This tool was originally built for use with [`Pr000xy`](https://github.com/0age/Pr000xy), including with [`Create2Factory`](https://github.com/0age/Pr000xy/blob/master/contracts/Create2Factory.sol) directly.

There is also an experimental OpenCL feature that can be used to search for addresses using a GPU. To give it a try, include a fourth parameter specifying the device ID to use, and optionally a fifth and sixth parameter to filter returned results by a threshold based on leading zero bytes and total zero bytes, respectively. By way of example, to perform the same search as above, but using OpenCL device 2 and only returning results that create addresses with at least four leading zeroes or six total zeroes, use `$ cargo run --release $FACTORY $CALLER $INIT_CODE_HASH 2 4 6` (you'll also probably want to try tweaking the `WORK_SIZE` parameter in `src/lib.rs`).

PRs welcome!
