zed_extensions_dir := "~/Library/Application Support/Zed/extensions"

# Remove all generated .wasm files and the cargo target directory
clean:
    rm -rf target/
    rm -rf grammars/hcl/
    find . -name '*.wasm' -not -path './.git/*' -delete

# Build the extension WASM binary
build:
    cargo build --target wasm32-wasip1 --release

# Install as a dev extension in Zed (opens Zed pointed at this directory)
dev-install:
    @open -a Zed
    @echo "Use 'zed: Install Dev Extension' from the command palette and select this directory:"
    @echo "  $(pwd)"
