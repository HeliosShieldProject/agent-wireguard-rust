# Helios Agent Wireguard Server
This project is an API server for managing WireGuard configurations, utilizing the Axum framework and interfacing directly with the WireGuard CLI. It provides a simple and efficient way to programmatically manipulate WireGuard settings, keys, and configurations. Whether you're automating VPN setup or managing multiple WireGuard instances, this server offers a powerful and flexible solution.

## Features
- API Management: Easily interact with WireGuard via RESTful API endpoints.
- Axum Framework: Built using the high-performance and ergonomic Axum web framework.
- WireGuard CLI Integration: Directly wraps and leverages the WireGuard command-line interface.
- Key Generation: Generate and manage public/private key pairs seamlessly.
- Configuration Handling: Create, update, and delete WireGuard configurations.

## Local Development

### Prerequisites

1. Installed Rust and Cargo. If you don't have it installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install). 
2. Installed and setup WireGuard or simply run setup [scripts/setup_wireguard.sh](scripts/setup_wireguard.sh).
3. Specify .env file following the example [.env.example](.env.example).

### Running the Server

```bash
cargo run
```
> Yeah, that's it! The server will be running on `http://<AGENT_HOST>:<AGENT_PORT>`.

## Remote Deployment (actions)

### Prerequisites

Setup github secrets following the table below:

| Secret Name | Description |
| --- | --- |
| HELIOS_SSH | SSH private key for the remote server |
| SSH_PORT | SSH port for the remote server |
| SSH_USER | SSH user for the remote server |
| SSH_HOST | IP address or hostname of the remote server |

## Remote Deployment (manual)

### Prerequisites

1. Install Wireguard running the setup script [scripts/setup_wireguard.sh](scripts/setup_wireguard.sh).
2. Allow firewall rules for the Agent Server `ufw allow <AGENT_PORT>`.
3. Download the latest release from the [releases page](https://github.com/HeliosShieldProject/agent-wireguard-rust/releases).
4. Specify .env file following the example [.env.example](.env.example).
5. Unzip the release and run the binary `./agent-wireguard-rust`.

