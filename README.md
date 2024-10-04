# Cloudflare DNS Updater

This Rust application updates a DNS record on Cloudflare with the current public IP address of your machine. The program
 retrieves the public IP address from a specified service and then makes an API call to Cloudflare to update the DNS record.

## Features
- Retrieves the current public IP address.
- Updates a specified DNS record on Cloudflare using the Cloudflare API.

## Prerequisites
- Rust installed on your system.
- A Cloudflare account and an API key.
- A Cloudflare zone and DNS record to update.

## Configuration
Create a `config.toml` file in the root of your project with the following format:

```toml
public_ip_service = "https://api.ipify.org"
cloudflare_api_key = "your_cloudflare_api_key"
zone_id = "your_zone_id"
dns_name = "record.zone.id"
```

## Usage
To run the program:

```sh
cargo run
```

This will update the specified DNS record on Cloudflare with your current public IP address.

## Dependencies
- `reqwest` for making HTTP requests.
- `serde` and `toml` for configuration parsing.
- `tokio` for asynchronous runtime.

## License
This project is licensed under the MIT License.