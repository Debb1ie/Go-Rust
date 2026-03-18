# ⚡ IronWeb — Rust Axum Web Server

A production-ready web server written in **pure Rust** using the **Axum 0.7** framework,
powered by the **Tokio** async runtime.

---

## Project Structure

```
ironweb/
├── Cargo.toml              # Dependencies & build config
├── src/
│   ├── main.rs             # Server entry point, router, middleware
│   ├── models/
│   │   └── mod.rs          # Shared data models (Serialize/Deserialize)
│   └── routes/
│       ├── mod.rs          # Route module declarations
│       └── api.rs          # All API handlers
└── static/
    └── index.html          # Frontend served by Axum
```

---

## Prerequisites

Install Rust via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Run (Development)

```bash
cargo run
```

Server starts at → **http://localhost:8080**

## Run (Production)

```bash
cargo build --release
./target/release/ironweb
```

The `--release` build enables full optimizations (LTO, dead-code stripping).
The binary is fully self-contained — no runtime dependencies.

---

## API Endpoints

| Method | Path              | Description              |
|--------|-------------------|--------------------------|
| GET    | `/api/`           | Server info & version    |
| GET    | `/api/health`     | Health check (JSON)      |
| GET    | `/api/projects`   | List all projects        |
| GET    | `/api/projects/:id` | Get project by ID      |
| POST   | `/api/contact`    | Submit contact form      |

### Example: Health Check

```bash
curl http://localhost:8080/api/health
```

```json
{ "status": "ok", "service": "ironweb", "lang": "Rust" }
```

### Example: List Projects

```bash
curl http://localhost:8080/api/projects | jq
```

### Example: POST Contact

```bash
curl -X POST http://localhost:8080/api/contact \
  -H "Content-Type: application/json" \
  -d '{"name":"Ada","email":"ada@rust.rs","message":"Hello Rust!"}'
```

---

## Stack

| Layer       | Crate                | Role                          |
|-------------|----------------------|-------------------------------|
| Runtime     | `tokio`              | Async executor                |
| Web         | `axum`               | Router + extractors           |
| Middleware  | `tower-http`         | CORS, compression, static files, tracing |
| Serialization | `serde` + `serde_json` | JSON in/out               |
| Logging     | `tracing` + `tracing-subscriber` | Structured logs     |
| Time        | `chrono`             | UTC timestamps in responses   |

---

## Logging

Set the `RUST_LOG` env var to control log verbosity:

```bash
RUST_LOG=debug cargo run          # verbose
RUST_LOG=ironweb=info cargo run   # only app logs
```
