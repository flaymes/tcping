# tcping
A simple tcp ping tool.

## Build
```shell
git clone https://github.com/flaymes/tcping.git
cd ~/tcping
cargo build --release
```

## Usage:
### ping IPv4 address
```shell
# tcp ping IPv4 address 
./tcping -H 39.156.66.10 -p 80
```
### ping IPv6 address
```shell
# tcp ping IPv6 address 
./tcping -6 -H 2a00:1450:400e:810::200e -p 80

```

