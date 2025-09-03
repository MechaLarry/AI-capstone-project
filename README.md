# AI Tutor (Rust + Axum)

An educational AI tutor API built with Rust, Axum, and Reqwest. It now uses Groq's OpenAI-compatible Chat Completions API.

## Features

- REST API with Axum
- JSON (serde) request/response
- Calls Groq Chat Completions (OpenAI-compatible)
- CORS enabled; simple static frontend in `static/`

## Prerequisites

- Rust (stable) and Cargo
- WSL (recommended on Windows) with `pkg-config` and `libssl-dev` installed
- A Groq API key

Install deps on Ubuntu/WSL:

```bash
sudo apt update
sudo apt install -y pkg-config libssl-dev
```

## Getting Started

1. Clone and enter the project
https://github.com/MechaLarry/AI-capstone-project.git

2. Create your .env

```bash
cp env.example .env
# then edit .env and set
# GROQ_API_KEY=your_groq_api_key_here
```

3. Run the server

```bash
cargo run
```

You should see logs like:

```
✅ .env file loaded successfully
🔑 Groq API key found: XXXXXXXX...YYYY
🌐 Server listening on 127.0.0.1:3000
```

4. Open the demo UI

- Navigate to `http://127.0.0.1:3000/` in your browser
- Ask a question; responses are generated via Groq

## API

### POST /api/ask

Request body:

```json
{
  "subject": "mathematics",
  "question": "What is the derivative of x^2?"
}
```

Response body:

```json
{
  "answer": "...",
  "subject": "mathematics",
  "status": "success"
}
```

### GET /health

Simple service health info.

## Configuration

- `.env` supports:
  - `GROQ_API_KEY` (required)
  - `PORT` (default 3000)
  - `HOST` (default 127.0.0.1)
  - `RUST_LOG` (optional)

## How it Works

- `src/handlers/ai_handler.rs`: HTTP handler for `/api/ask`; reads `GROQ_API_KEY` and calls the client
- `src/services/langchain_service.rs`: Groq client using `https://api.groq.com/openai/v1/chat/completions`
- `src/models/`: request/response DTOs
- `src/main.rs`: server, routes, CORS, static files

## Changing the Model

Default model:

- `meta-llama/llama-4-scout-17b-16e-instruct`

To change, edit `model` in `src/services/langchain_service.rs`.

## Troubleshooting

- 500 Internal Server Error in UI

  - Check server logs in the terminal running `cargo run`
  - Ensure `.env` exists and `GROQ_API_KEY` is set
  - Verify internet connectivity and that your Groq key is active

- `cargo: command not found` on Windows

  - Use WSL and run commands there
  - Or install Rust for Windows from https://rustup.rs

- SSL or OpenSSL build errors
  - On WSL/Ubuntu, install `pkg-config` and `libssl-dev` (see prerequisites)

## Development

Run tests:

```bash
cargo test
```

Format & lint suggestions:

```bash
cargo fmt
cargo clippy
```

## License

MIT
