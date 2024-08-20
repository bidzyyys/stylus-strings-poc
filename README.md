# Stylus-EVM-Precompiles

The repository contains playground for testing Strings in Stylus smart contracts (`v0.5.2`).

## Build

```sh
cargo stylus check -e=<rpc-endpoint>
```

## Export ABI

```sh
cargo stylus export-abi
```

## Deploy

```sh
cargo stylus deploy --private-key=<priv-key> -e=<rpc-endpoint>
```
