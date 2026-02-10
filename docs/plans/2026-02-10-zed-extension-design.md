# Zed Extension for PaletteSwap Theme Files

**Date:** 2026-02-10  
**Status:** Design Complete

## Overview

A Zed editor extension that provides syntax highlighting and LSP support for `.pstheme` files used by the PaletteSwap theme generation tool.

## Architecture

The extension provides two main capabilities:

1. **Syntax highlighting** via Tree-sitter grammar for HCL files with PaletteSwap-specific constructs
2. **LSP integration** that launches `pstheme-lsp` for language features

**Structure:**
```
extension/
├── extension.toml          # Extension manifest
├── languages/
│   └── pstheme/
│       ├── config.toml     # Language configuration
│       └── highlights.scm  # Syntax highlighting queries
└── src/
    └── lib.rs              # Extension entry point (LSP setup)
```

The extension registers `.pstheme` files as a new language "PaletteSwap Theme" that extends HCL. Zed uses Tree-sitter's HCL grammar for parsing, with custom highlight queries for PaletteSwap-specific blocks (meta, palette, theme, syntax, ansi).

The LSP binary `pstheme-lsp` will be launched as a standalone process, communicating via stdin/stdout. Zed's extension API handles the LSP client lifecycle.

## Components & Data Flow

**Extension Components:**

1. **Language Definition** (`languages/pstheme/config.toml`):
   - Name: "PaletteSwap Theme"
   - File extensions: `.pstheme`
   - Grammar: HCL (bundled with fallback to Zed's built-in)
   - Tab size: 2 spaces (HCL convention)

2. **Syntax Highlighting** (`languages/pstheme/highlights.scm`):
   - Standard HCL highlighting (blocks, attributes, strings, comments)
   - PaletteSwap-specific highlights for block names: `meta`, `palette`, `theme`, `syntax`, `ansi`
   - Special highlighting for hex color values (`#RRGGBB`)
   - Highlight for `brighten()` function calls

3. **LSP Client** (`src/lib.rs`):
   - Spawns `pstheme-lsp` from PATH
   - Handles initialization and message passing
   - Supports all LSP features the server provides (diagnostics, completions, hover)

**Data Flow:**
```
.pstheme file → Zed editor → Tree-sitter parser → Syntax highlighting
                                    ↓
                              LSP client (extension)
                                    ↓
                              pstheme-lsp (stdin/stdout)
                                    ↓
                              Diagnostics, completions, hover
```

## Error Handling & Edge Cases

**LSP Binary Not Found:**
- Check PATH for `pstheme-lsp` on extension activation
- If missing: show notification with install instructions
- Gracefully degrade to syntax highlighting only

**LSP Crashes/Errors:**
- Auto-restart LSP if it crashes (with backoff)
- Log errors to Zed's extension logs
- Continue providing syntax highlighting

**Invalid .pstheme Files:**
- LSP provides diagnostics for HCL syntax errors
- LSP provides diagnostics for semantic errors (undefined palette references, etc.)
- Invalid files still get syntax highlighting

**Large Files:**
- LSP should handle files up to 5KB (expected max size)
- Tree-sitter incremental parsing keeps editor responsive

**Multiple Files Open:**
- Single LSP process handles all open .pstheme files
- Proper workspace/didChange notifications

## Testing Strategy

**Manual Testing:**
1. Open a `.pstheme` file and verify syntax highlighting works
2. Verify LSP features: hover on palette references, completions, diagnostics
3. Test error cases: rename `pstheme-lsp` binary, verify graceful degradation

**Test Files:**
Create sample files in `test/fixtures/`:
- `valid.pstheme` - Complete theme with all blocks
- `invalid-syntax.pstheme` - Malformed HCL for error testing
- `minimal.pstheme` - Bare minimum valid file

**CI/CD:**
- Build extension with `cargo build --release`
- Verify extension loads in Zed (extension dev mode)
- No automated LSP testing needed (LSP has its own tests)

## Summary

The extension provides:
- Syntax highlighting for HCL-based .pstheme files
- LSP integration via `pstheme-lsp` binary
- Graceful degradation if LSP unavailable
- Support for files up to 5KB

## References

- PaletteSwap project: `/Users/echo/git/github.com/jsvensson/paletteswap`
- LSP binary: `pstheme-lsp` (available in PATH)
- Zed extension docs: https://zed.dev/docs/extensions
