# ASIMOV Ethereum Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-ethereum-module.svg)](https://crates.io/crates/asimov-ethereum-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-ethereum-module.svg)](https://pypi.org/project/asimov-ethereum-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-ethereum-module.svg)](https://rubygems.org/gems/asimov-ethereum-module)
[![Package on NPM](https://img.shields.io/npm/v/asimov-ethereum-module.svg)](https://npmjs.com/package/asimov-ethereum-module)

[ASIMOV] module for data import from the [Ethereum] blockchain network.

## ✨ Features

- Imports structured data from [Ethereum] blocks and transactions.
- Supports mainnet, [Sepolia], and [Holesky] networks.
- Collects raw JSON data via public Ethereum JSON-RPC endpoints.
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from PyPI

```bash
pip install -U asimov-ethereum-module
```

### Installation from RubyGems

```bash
gem install asimov-ethereum-module
```

### Installation from NPM

```bash
npm install -g asimov-ethereum-module
```

### Installation from Source Code

```bash
cargo install asimov-ethereum-module
```

## 👉 Examples

### Fetching Mainnet Blocks

```bash
asimov-ethereum-fetcher ethereum://mainnet/18000000
```

### Fetching Sepolia Testnet Blocks

```bash
asimov-ethereum-fetcher ethereum://sepolia/123456
```

### Fetching Holesky Testnet Blocks

```bash
asimov-ethereum-fetcher ethereum://holesky/123456
```

## 📚 Reference

### Installed Binaries

- `asimov-ethereum-fetcher`: collects JSON data from Ethereum JSON-RPC endpoints

### Supported Networks

| Network | URL Scheme | RPC Endpoint |
|---------|-----------|--------------|
| Mainnet | `ethereum://mainnet/<height>` | ethereum.publicnode.com |
| Sepolia | `ethereum://sepolia/<height>` | ethereum-sepolia.publicnode.com |
| Holesky | `ethereum://holesky/<height>` | ethereum-holesky.publicnode.com |

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-ethereum-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-ethereum-module&text=asimov-ethereum-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-ethereum-module&title=asimov-ethereum-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-ethereum-module&t=asimov-ethereum-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-ethereum-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-ethereum-module)

[ASIMOV]: https://github.com/asimov-platform
[Ethereum]: https://ethereum.org
[Holesky]: https://github.com/eth-clients/holesky
[NPM]: https://npmjs.org
[Python]: https://python.org
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
[Sepolia]: https://sepolia.dev
