# Oxdown

An opinionated static site generator for markdown articles.

## Features

- Convention-based article directories (`YYYY-MM-DD-slug/`)
- Markdown to HTML conversion
- Automatic static file copying (images, etc.)
- Dark mode toggle with localStorage persistence
- Optional comment/discussion links per article
- Configuration via JSON file
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

## Configuration

Oxdown requires a configuration file in JSON format. You can specify it either as a command-line argument or via the `OXDOWN_CONFIG` environment variable.

Create a `config.json` file:

```json
{
  "input_directory": "./articles",
  "output_directory": "./dist",
  "template_directory": "templates/default",
  "author_name": "John Doe",
  "author_url": "https://example.com/author"
}
```

**Required fields:**
- `input_directory`: Path to the directory containing article directories

**Optional fields:**
- `output_directory`: Where to generate the site (default: `"dist"`)
- `template_directory`: Path to custom templates (default: `"templates/default"`)
- `author_name`: Author name to display at the bottom of articles (optional)
- `author_url`: URL to link the author name to (optional, requires `author_name`)

## Usage

Create article directories following the naming convention:

```
articles/
├── 2024-01-15-hello-world/
│   ├── index.md
│   ├── index.json       (optional)
│   └── diagram.png
└── 2024-01-20-another-post/
    ├── index.md
    └── index.json       (optional)
```

Each `index.md` must start with a level 1 heading:

```markdown
# Article Title

Your content here...
```

### Optional Article Metadata

You can add an optional `index.json` file alongside your `index.md` to specify additional metadata:

```json
{
  "comment_url": "https://twitter.com/yourhandle/status/123456789"
}
```

When `comment_url` is specified, a "Follow me or comment" link will be displayed at the end of the article.

### Generate the site

With a config file argument:

```bash
oxdown config.json
```

Or using the environment variable:

```bash
export OXDOWN_CONFIG=config.json
oxdown
```

## Development

See available development commands in the `justfile`.

## License

MIT
