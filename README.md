# flare-at
Point a Cloudflare website to an IP address using Type A DNS records.

## Requirements
- Get the zone ID from Cloudflare dashboard overview tab.
- Create an API token with permission to read and write DNS records.
- Create a Type A DNS record for the domain. This utility will not create a DNS record, it will merely update an existing one.

## Usage
1. Build the project with `cargo build --release`. Drop the `release` flag to compile faster without optimizations.
1. Use it like this: `./target/release/flare-at -t <token> -z <zone> -n example.com <ip>`.
1. To see what each option means run `./target/release/flare-at -h`
