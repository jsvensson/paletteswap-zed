# Remove all generated .wasm files and the cargo target directory
clean:
    rm -rf target/
    rm -rf grammars/hcl/
    find . -name '*.wasm' -not -path './.git/*' -delete

# Build the extension WASM binary
build:
    cargo build --target wasm32-wasip1 --release

# Path to local paletteswap repo

paletteswap_repo := "../paletteswap"

# Build and install the LSP binary from local paletteswap source
lsp-install repo=paletteswap_repo:
    cd {{ repo }} && go build -o "{{ justfile_directory() }}/pstheme-lsp" ./cmd/pstheme-lsp
    @echo "Installed pstheme-lsp from {{ repo }}"

# Install as a dev extension in Zed (opens Zed pointed at this directory)
dev-install:
    @open -a Zed
    @echo "Use 'zed: Install Dev Extension' from the command palette and select this directory:"
    @echo "  $(pwd)"
