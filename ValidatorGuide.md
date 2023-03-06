# How to run an EXZO Validator
## Build EXZO
### **1. Install rustc, cargo and rustfmt.**

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup component add rustfmt
```

Please sure you are always using the latest stable rust version by running:

```bash
$ rustup update
```

On Linux systems you may need to install libssl-dev, pkg-config, zlib1g-dev, etc.  On Ubuntu:

```bash
$ sudo apt-get update
$ sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang make cmake protobuf-compiler -y
```

On Mac M1s, make sure you set up your terminal & homebrew [to use](https://5balloons.info/correct-way-to-install-and-use-homebrew-on-m1-macs/) Rosetta. You can install it with:

```bash
$ softwareupdate --install-rosetta
```

### **2. Download the source code.**

```bash
$ git clone https://github.com/ExzoNetwork/Exzo-Network-Blockchain.git
$ cd Exzo-Network-Blockchain
```

### **3. Build.**

```bash
$ cargo build --release
```

## Validator Setup
### Configure EXZO CLI
**Mainnet**
```
exzo config set --url https://mainnet.exzo.technology
```

**Testnet**
```
exzo config set --url https://testnet.exzo.technology
```

### Generate Validator Identity
