# dnsqry

A fast and simple DNS query tool written in Rust.

## Description

`dnsqry` is a command-line DNS lookup utility that provides clean, formatted output for various DNS record types. It's designed to be simple, fast, and easy to use for both quick lookups and scripting.

## Installation

### Build from source

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd dnsqry
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. The binary will be available at `target/release/dnsqry`

5. Optionally, install it to your PATH:
   ```bash
   cargo install --path .
   ```

## Usage

```bash
dnsqry <domain> <record_type>
```

### Arguments

- `domain`: The domain name to query (e.g., `google.com`, `example.org`)
- `record_type`: The DNS record type to query (A, AAAA, NS, MX, TXT, CNAME, SOA, PTR, etc.)

### Examples

Query A record (IPv4 address):
```bash
$ dnsqry google.com A
A google.com. 3m01s   142.251.222.206
```

Query NS records (name servers):
```bash
$ dnsqry google.com NS
NS google.com. 39m57s   "ns1.google.com."
NS google.com. 39m57s   "ns2.google.com."
NS google.com. 39m57s   "ns3.google.com."
NS google.com. 39m57s   "ns4.google.com."
```

Query AAAA record (IPv6 address):
```bash
$ dnsqry google.com AAAA
AAAA google.com. 2m15s   2607:f8b0:4004:c1b::65
```

Query MX records (mail servers):
```bash
$ dnsqry google.com MX
MX google.com. 10m30s   10 "smtp.google.com."
```

Query TXT records:
```bash
$ dnsqry google.com TXT
TXT google.com. 5m00s   "v=spf1 include:_spf.google.com ~all"
```

Query CNAME record:
```bash
$ dnsqry www.github.com CNAME
CNAME www.github.com. 1h00m00s   "github.com."
```

## Supported Record Types

- **A**: IPv4 address records
- **AAAA**: IPv6 address records
- **NS**: Name server records
- **MX**: Mail exchange records
- **TXT**: Text records
- **CNAME**: Canonical name records
- **SOA**: Start of authority records
- **PTR**: Pointer records (for reverse DNS)
- And other standard DNS record types

## Output Format

The output format is designed to be both human-readable and script-friendly:

```
<RECORD_TYPE> <DOMAIN> <TTL>   <DATA>
```

Where:
- **RECORD_TYPE**: The DNS record type (A, NS, MX, etc.)
- **DOMAIN**: The queried domain name
- **TTL**: Time-to-live in human-readable format (e.g., `3m01s`, `1h30m45s`)
- **DATA**: The record data (IP addresses, hostnames in quotes, etc.)

## Error Handling

The tool provides clear error messages for common issues:

- Invalid record types
- DNS resolution failures
- Network connectivity problems
- Malformed domain names

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [trust-dns-resolver](https://crates.io/crates/trust-dns-resolver) - DNS resolution library

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
