# udp-ping

UDP ping utility, written in Rust

## Usage

This library produces 2 binaries, called `client` and `server`.

Run `server` first in one tab:

`cargo run --bin server`

Then run `client` in another tab:

`cargo run --bin client`

Client will send 1000 UDP pings to the server's address, check for replies,
the print out the number of packets transmitted and the packet loss (if any).

## Example run

```sh
% cargo run --bin client
1000 packets transmitted, 1000 packets received, 0.0% packet loss
```

