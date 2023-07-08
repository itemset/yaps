# y.a.p.s

> **Warning**
> POC (Proof-of-Concept) program,

Yet another port sniffing tool written in rust.

# setup

- clone the git repository

`git clone https://github.com/itemset/yaps.git`

- install dependencies

`cargo install`

- build application

`cargo build`

# usage

```
./yaps

Yet another port sniffing tool

Usage: yaps [OPTIONS] --ip-address <IP_ADDRESS>

Options:
  -i, --ip-address <IP_ADDRESS>  IP address to scan open ports on
  -t, --threads <THREADS>        Number of threads to scan a given IP address on [default: 1]
  -v, --verbose                  Verbose logging
  -h, --help                     Print help
  -V, --version                  Print version
```

## scan an address

```
./yaps -i ADDRESS -t THREAD_NUM -v
```