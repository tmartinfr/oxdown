# Install oxdown locally
install:
    cargo install --path .

# Run clippy linter
lint:
    cargo clippy

# Build the project
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run the project (requires input directory argument)
run INPUT OUTPUT="dist":
    cargo run -- {{INPUT}} --output {{OUTPUT}}

# Lint, then build
check: lint build

# Clean build artifacts
clean:
    cargo clean

# Install miniserve
install-miniserve:
    cargo install miniserve

# Serve the generated site with miniserve
serve OUTPUT="dist":
    @which miniserve > /dev/null || (echo "miniserve not found. Run: just install-miniserve" && exit 1)
    miniserve --index index.html {{OUTPUT}}
