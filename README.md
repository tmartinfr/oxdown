# Oxdown

An opinionated static site generator for markdown articles.

## Features

- Convention-based article directories (`YYYY-MM-DD-slug/`)
- Markdown to HTML conversion
- Automatic static file copying (images, etc.)
- Minimal default design
- Fast Rust implementation

## Installation

```bash
just install
```

Or directly with cargo:

```bash
cargo install --path .
```

## Usage

Create article directories following the naming convention:

```
articles/
├── 2024-01-15-hello-world/
│   ├── index.md
│   └── diagram.png
└── 2024-01-20-another-post/
    └── index.md
```

Each `index.md` must start with a level 1 heading:

```markdown
# Article Title

Your content here...
```

Generate the site:

```bash
oxdown ./articles/ --output dist
```

## Development

```bash
# Run linter
just lint

# Build the project
just build

# Generate and serve a test site
just run ./articles/
just serve
```

## License

MIT
