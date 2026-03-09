# Proof of Work

A simple command-line tool for generating, solving proof-of-work challenges, and wrapping command execution.

## Installation

```bash
cargo build --release
```

## Usage

Generate a challenge.

```bash
pow generate <difficulty>
```

Solve a challenge.

```bash
pow solve <challenge>
```

Check a solution.

```bash
pow check <challenge> <solution>
```

Run command after solving.

```bash
pow run <difficulty> -- <command>
```

## Use in your challenge

Example Dockerfile:

```Dockerfile
FROM ghcr.io/pj-newstar/proof-of-work:latest AS pow

FROM alpine:latest

ENV FLAG=flag{test_flag}

# copy the pow binary
COPY --from=pow /usr/local/bin/pow /usr/local/bin/pow

# install socat
RUN apk update && apk add --no-cache coreutils socat

# add the challenge binary
COPY ./pwn /var/chal/pwn
RUN chmod +x /var/chal/pwn

# CMD ["tcpserver", "-v", "-D", "-c", "50", "0", "1337", "/usr/local/bin/pow", "run", "15360", "--", "/var/chal/pwn"]
CMD ["socat", "TCP4-LISTEN:1337,reuseaddr,fork", "EXEC:/usr/local/bin/pow run 15360 -- /var/chal/pwn"]

EXPOSE 1337
```

## References

- [kCTF PoW](https://goo.gle/kctf-pow)
- [kCTF PoW Rust Crate](https://github.com/Aplet123/kctf-pow)

## License

Copyright (c) Cnily03. All rights reserved.

Licensed under the [MIT License](LICENSE).
