
# DNSSD_2

<div id="header" align="center">
Discussion Branch
<img src="https://c.neevacdn.net/image/fetch/s--NpVnXOWX--/https%3A//www.rustacean.net/more-crabby-things/rustdocs.png?savepath=rustdocs.png" width="200" height="200" />



![MIT License](https://img.shields.io/badge/License-MIT-green.svg)

![Checks](https://img.shields.io/github/checks-status/soundprojects/dns_sd2/master)

</div>
This crate is attempting to correctly implement the functionality defined for the MDNS and DNSSD protocols.
Please read on for the scope of this project and how to include it in your project.

This crate functions on its own by using sockets and UDP Mdns Messages
It does not depend on a Bonjour or Avahi resolver

Feel free to submit issues if this crate does not correctly implement the mentioned features!


## Documentation

[Documentation](https://linktodocumentation)


## Features

- 📡 Probing and Announcing        
- 🗣 Querying and Responding
- 👋 Goodbye Packets
- 📦 Name Compression
- ✍️ Service Registration
- 🔎 Service Browsing


## Scope
This crate has the goal of implementing:
- [RFC 6762 Multicast DNS](https://www.rfc-editor.org/rfc/rfc6762)
- [RFC 6763 DNS-Based Service Discovery](https://www.rfc-editor.org/rfc/rfc6763)

as good as possible for providing a stable and proper implementation for a `browse()` and `register()` function

## Not Included
- Acting as a resolver / cache for services you did not create
- Service Types outside of DNSSD Scope (MX MD NS etc) -> Enums are there but implementation is up to you


## Running Tests

To run tests, run the following command

```bash
  cargo test
```


## Usage/Examples

```javascript
MagicalCodeExample(){}
```

