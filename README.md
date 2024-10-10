# ETSI GS QKD 014 client command-line program and library for Rust and C

[![DOI](https://data.4tu.nl/v3/datasets/3618fc0a-6b89-46c3-8199-5cf5bdb46a29/doi-badge.svg)](https://doi.org/10.4121/3618fc0a-6b89-46c3-8199-5cf5bdb46a29)

Quantum Key Distribution (QKD) hardware provides cryptographic keys that can be used to secure confidential data. This software allows to request keys exchanged using QKD from QKD hardware or a Key Management System by following the ETSI GS QKD 014 standard. It is then up to the user of this library to use these keys to encrypt data. There are 3 ways to use the client: as a command-line program, as a Rust crate or as a C library.

## Dependencies

Install [Rust](https://www.rust-lang.org/tools/install).
Then install libsodium:

```bash
sudo apt install libsodium-dev
```

## Command-line program

### Installation

```bash
cd binary
cargo install --path .
```

### Usage

Retrieving KME status:

```bash
$ etsi014-cli --host kms.example.org --port 443 --key client-1.key --cert client-1.crt --server-ca server-ca.crt --target-sae-id client-2 status
source_KME_ID=kms-1.example.org
target_KME_ID=kms-2.example.org
source_SAE_ID=client-1
target_SAE_ID=client-2
key_size=256
...
```

Requesting new keys:

```
$ etsi014-cli --host kms.example.org --port 443 --key client-1.key --cert client-1.crt --server-ca /usr/share/ca-certificates/mozilla/ISRG_Root_X1.crt --target-sae-id client-2 get-keys --amount 3 --key-size 256
851884a2-57c3-4b83-876e-6de27882d003=1ca3bcde45f880df267c0c70110921c77c442b28400e2f67ba2f84d408aa2a1c
16771d3f-994b-4850-aa5e-86138544a4a6=4b1a4cd007e672f8e662d72e8c62c146c36485668cfbddb2245c113d551b41fa
b81bfeec-c35f-45e1-a394-361da46f3dcb=1b7bc8a5c3a4a994bb6e1e69005c595c206116e381f8670b168024a028d21277
```

Requesting keys by UUID:

```bash
$ etsi014-cli --host kms.example.org --port 443 --key client-2.key --cert client-2.crt --server-ca /usr/share/ca-certificates/mozilla/ISRG_Root_X1.crt --target-sae-id client-1 get-keys-by-ids --ids=851884a2-57c3-4b83-876e-6de27882d003,16771d3f-994b-4850-aa5e-86138544a4a6,b81bfeec-c35f-45e1-a394-361da46f3dcb
851884a2-57c3-4b83-876e-6de27882d003=1ca3bcde45f880df267c0c70110921c77c442b28400e2f67ba2f84d408aa2a1c
16771d3f-994b-4850-aa5e-86138544a4a6=4b1a4cd007e672f8e662d72e8c62c146c36485668cfbddb2245c113d551b41fa
b81bfeec-c35f-45e1-a394-361da46f3dcb=1b7bc8a5c3a4a994bb6e1e69005c595c206116e381f8670b168024a028d21277
```

## Rust crate

* [Usage example in Rust](binary/src/main.rs)

## Shared library with C API

### Installation

```bash
cargo build --release --lib
sudo cp target/release/libetsi014_client.so /usr/local/lib/libetsi014_client.so
# Header for development
sudo mkdir /usr/local/include/etsi014-client/
sudo cp library/c/etsi014-client.h /usr/local/include/etsi014-client/etsi014-client.h
```

### Usage

* [Usage example in C](examples/c/)

## Documentation

* [ETSI GS QKD 014 v1.1.1](https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_qkd014v010101p.pdf)

## Acknowledgements

This project is funded by the [Dutch Research Council](https://www.nwo.nl/en) under the [FIQCS project](https://www.fiqcs.nl/) (NWA.1436.20.005).

## License

This project is licensed under the [MIT license](LICENSE).

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed under the [MIT license](LICENSE) and the [Apache-2.0 license](https://www.apache.org/licenses/LICENSE-2.0.txt), without any additional terms or conditions.
