# Unchain - Cargo Workspace

This project is organized as a Cargo workspace with three main components:

## Project Structure

```
unchain/
├── Cargo.toml          # Workspace configuration
├── client/             # Client application
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── server/             # Server application
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── shared/             # Shared library
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── cid.rs
```

## Components

### 🔧 Shared Library (`unchain-shared`)
Contains common functionality used by both client and server:
- CID parsing and validation
- Common data structures
- Error types
- Utilities

### 📱 Client (`unchain-client`)
Command-line client application that:
- Accepts CID strings as input
- Parses and validates CIDs
- Can communicate with the server
- Provides CLI interface using clap

### 🖥️ Server (`unchain-server`)
HTTP server application that:
- Exposes REST API endpoints
- Handles CID parsing requests
- Built with Axum web framework
- Provides health check endpoint

## Building and Running

### Build all components
```bash
cargo build
```

### Run the server
```bash
cargo run --bin server
```

### Run the client
```bash
cargo run --bin client <cid_string>
```

### Run with server URL
```bash
cargo run --bin client --server http://localhost:3000 <cid_string>
```

## API Endpoints

### Server Endpoints
- `GET /health` - Health check
- `GET /parse?cid=<cid>` - Parse a CID string

### Example Usage
```bash
# Start the server
cargo run --bin server

# In another terminal, test the client
cargo run --bin client bafybeigdyrztpyzbs7thpwq6qqc7rsn2s3ye2q6yrvtu5foaunvjo4brga

# Or test the server directly
curl "http://localhost:3000/parse?cid=bafybeigdyrztpyzbs7thpwq6qqc7rsn2s3ye2q6yrvtu5foaunvjo4brga"
```

## Development

### Adding Dependencies
- For shared functionality: Add to `shared/Cargo.toml`
- For client-specific features: Add to `client/Cargo.toml`
- For server-specific features: Add to `server/Cargo.toml`

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific component
cargo test -p unchain-shared
cargo test -p unchain-client
cargo test -p unchain-server
```
