# 🧠 AI Tutor - Learn Anything with AI

A web-based AI tutoring application built with Rust and powered by LangChain API.

## Features

- 📚 Multiple subject support (Math, Science, History, Literature, etc.)
- 🤖 AI-powered explanations tailored to each subject
- 🎯 Simple, clean web interface
- ⚡ Fast Rust backend with Axum web framework

## Quick Start

1. **Clone and setup:**

```bash
git clone <your-repo>
cd ai-tutor
```

2. **Set up environment:**

```bash
cp env.example .env
# Edit .env and add your LangChain API key
```

3. **Run the application:**

```bash
cargo run
```

4. **Open your browser:**

```
http://localhost:3000
```

## API Keys

You need a LangChain API key:

- Set `LANGCHAIN_API_KEY` in your `.env` file

## Project Structure

```
src/
├── main.rs              # Server setup and routing
├── lib.rs               # Library exports
├── handlers/            # Request handlers
│   ├── mod.rs          # Handler module exports
│   └── ai_handler.rs   # AI question processing
├── services/            # Business logic
│   ├── mod.rs          # Service module exports
│   ├── langchain_service.rs  # AI API integration
│   └── prompt_service.rs     # Subject-specific prompts
├── models/              # Data structures
│   ├── mod.rs          # Model module exports
│   ├── request.rs      # Request types
│   └── response.rs     # Response types
├── utils/               # Utility functions
│   ├── mod.rs          # Utility module exports
│   ├── validation.rs   # Input validation
│   └── config.rs       # Configuration management
├── static/              # Frontend assets
│   ├── index.html      # Main interface
│   └── script.js       # Frontend logic
├── tests/               # Test files
│   └── integration_tests.rs
├── examples/            # Example usage
│   └── test_client.rs  # API testing client
└── scripts/             # Build and setup scripts
    ├── setup.sh        # Project setup
    └── test.sh         # Test runner
```

## Usage

1. Select a subject from the dropdown
2. Type your question
3. Click "Ask AI Tutor"
4. Get personalized explanations!

## Development

- `cargo run` - Start development server
- `cargo test` - Run tests
- `cargo build --release` - Build for production
- `cargo run --example test_client` - Test API endpoints

## Technologies Used

- **Backend**: Rust, Axum, Tokio
- **AI**: LangChain API
- **Frontend**: HTML, CSS, JavaScript
- **HTTP Client**: Reqwest
- **Validation**: Regex
- **Configuration**: Environment variables

Built by Larrzone Solutions LTD as part of Moringa AI Capstone Project.
