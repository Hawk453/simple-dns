# Simple-DNS

<img alt="GitHub" src="https://img.shields.io/github/license/Hawk453/simple-dns?style=flat-square"> <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/Hawk453/simple-dns?style=flat-square&color=44AA44">

## What is Simple-DNS?

A straightforward and intelligible DNS Server written in safe rust to provide a quick and easy understanding of DNS.
**Disclaimer:** This project is not meant for production usecase. 

## Get Started

This assumes that you have Rust stable installed. These presume that the simple-dns repo have already been synced to the local system. In the simple-dns directory, execute the following commands in the terminal:

```shell
cargo build
cargo run
```

## Goals and ToDo

- [x] Stub Resolver
- [ ] Recursive Resolve
- [ ] DNS compression when writing
- [ ] Authoritative Server
- [ ] Caching Server
- [ ] Forwarding Queries to a DNS Server of choice
- [ ] Protect against DDOS attacks (to a degree)
- [ ] TTD

## Credit

Thanks [Emil](https://github.com/EmilHernvall) for a guide on how to build a DNS.

## Contribution

Feel free to comment and open PRs to contribute to this project under the specified license terms.

## Contact

Author: [Saksham Madan](mailto:contact_saksham.unserialize@simplelogin.co) 