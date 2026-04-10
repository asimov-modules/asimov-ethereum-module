# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1] - 2026-04-10

### Added

- `asimov-ethereum-fetcher` CLI binary for fetching Ethereum block data by URL.
- `BlockUrl` type for parsing and resolving Ethereum block URLs (e.g. `eth://mainnet/12345678`).
- Support for Ethereum networks: `mainnet`, `sepolia`, and `holesky` via public RPC endpoints.
- JSON and JSONL output formats via `--output jsonl` flag.
- `--version` and `--license` flags via `clientele` standard options.
- Integration with `asimov-module` for argument parsing and optional tracing support.
