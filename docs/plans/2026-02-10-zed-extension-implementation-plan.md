# Zed Extension for PaletteSwap Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a Zed extension that provides syntax highlighting and LSP support for `.pstheme` files used by the PaletteSwap theme generation tool.

**Architecture:** The extension uses Zed's extension API (Rust/WASM) to register a new language "PaletteSwap Theme" with HCL grammar, custom syntax highlighting queries, and LSP integration via the `pstheme-lsp` binary.

**Tech Stack:** Rust, Tree-sitter (HCL grammar), Zed Extension API, LSP

---

## Prerequisites

- Worktree location: `/Users/echo/git/github.com/jsvensson/paletteswap-zed/.worktrees/zed-extension`
- LSP binary available at: `/Users/echo/go/bin/pstheme-lsp`
- PaletteSwap format documentation: See design doc at `docs/plans/2026-02-10-zed-extension-design.md`

---

### Task 1: Create Extension Manifest

**Files:**
- Create: `extension.toml`

**Step 1: Create extension.toml**

```toml
id = "paletteswap-theme"
name = "PaletteSwap Theme"
version = "0.1.0"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "Syntax highlighting and LSP support for PaletteSwap .pstheme files"
repository = "https://github.com/jsvensson/paletteswap-zed"

[grammars.hcl]
repository = "https://github.com/tree-sitter-grammars/tree-sitter-hcl"
rev = "e936d3fef8bacf23f0f243b5a8d405b2714d6ff8"

[language_servers.pstheme-lsp]
name = "PaletteSwap LSP"
languages = ["PaletteSwap Theme"]
```

**Step 2: Verify file created**

Run: `cat extension.toml`
Expected: File contents displayed

**Step 3: Commit**

```bash
git add extension.toml
git commit -m "feat: add extension manifest"
```

---

### Task 2: Create Language Configuration

**Files:**
- Create: `languages/pstheme/config.toml`

**Step 1: Create directory structure**

Run: `mkdir -p languages/pstheme`

**Step 2: Create config.toml**

```toml
name = "PaletteSwap Theme"
grammar = "hcl"
path_suffixes = ["pstheme"]
line_comments = ["# "]
tab_size = 2
```

**Step 3: Verify file created**

Run: `cat languages/pstheme/config.toml`
Expected: File contents displayed

**Step 4: Commit**

```bash
git add languages/pstheme/config.toml
git commit -m "feat: add language configuration"
```

---

### Task 3: Create Syntax Highlighting Queries

**Files:**
- Create: `languages/pstheme/highlights.scm`

**Step 1: Create highlights.scm**

```scheme
; HCL basics
(string) @string
(number) @number
(bool) @boolean
(comment) @comment

; Block types - PaletteSwap specific
(block
  (identifier) @keyword
  (#match? @keyword "^(meta|palette|theme|syntax|ansi)$"))

; Hex color values
(string
  (template_literal) @string.special
  (#match? @string.special "^#[0-9a-fA-F]{6}$"))

; Function calls - brighten()
(function_call
  (identifier) @function
  (#match? @function "^brighten$"))

; Attribute names
(attribute
  (identifier) @property)

; Block identifiers
(block
  (identifier) @type)

; Operators
("=") @operator
("{") @punctuation.bracket
("}") @punctuation.bracket
("(") @punctuation.bracket
(")") @punctuation.bracket
```

**Step 2: Verify file created**

Run: `cat languages/pstheme/highlights.scm`
Expected: File contents displayed

**Step 3: Commit**

```bash
git add languages/pstheme/highlights.scm
git commit -m "feat: add syntax highlighting queries"
```

---

### Task 4: Create Rust Extension Code

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "paletteswap-zed"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.1.0"
```

**Step 2: Create src directory**

Run: `mkdir -p src`

**Step 3: Create src/lib.rs**

```rust
use zed_extension_api as zed;

struct PaletteSwapExtension;

impl zed::Extension for PaletteSwapExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "pstheme-lsp".to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(PaletteSwapExtension);
```

**Step 4: Verify files created**

Run: `cat Cargo.toml && echo "---" && cat src/lib.rs`
Expected: Both files displayed

**Step 5: Commit**

```bash
git add Cargo.toml src/lib.rs
git commit -m "feat: add Rust extension code with LSP support"
```

---

### Task 5: Create Test Fixtures

**Files:**
- Create: `test/fixtures/valid.pstheme`
- Create: `test/fixtures/minimal.pstheme`

**Step 1: Create test directory**

Run: `mkdir -p test/fixtures`

**Step 2: Create valid.pstheme**

```hcl
meta {
  name       = "Test Theme"
  author     = "Test Author"
  appearance = "dark"
}

palette {
  base    = "#191724"
  surface = "#1f1d2e"
  text    = "#e0def4"
  accent  = "#eb6f92"

  highlight {
    low  = "#21202e"
    mid  = "#403d52"
    high = "#524f67"
  }
}

theme {
  background = palette.base
  foreground = palette.text
  cursor     = palette.highlight.high
}

syntax {
  keyword = palette.accent
  string  = "#f6c177"

  comment {
    color  = palette.surface
    italic = true
  }
}

ansi {
  black = palette.surface
  red   = palette.accent
}
```

**Step 3: Create minimal.pstheme**

```hcl
meta {
  name = "Minimal"
}

palette {
  base = "#000000"
}
```

**Step 4: Verify files created**

Run: `ls -la test/fixtures/`
Expected: valid.pstheme and minimal.pstheme listed

**Step 5: Commit**

```bash
git add test/
git commit -m "test: add test fixtures"
```

---

### Task 6: Build Extension

**Files:**
- Build: `Cargo.toml` and `src/lib.rs`

**Step 1: Build the extension**

Run: `cargo build --release --target wasm32-wasi`

Expected: Successful compilation, `.wasm` file created in `target/wasm32-wasi/release/`

**Step 2: Verify output**

Run: `ls -la target/wasm32-wasi/release/*.wasm`
Expected: `paletteswap_zed.wasm` file exists

**Step 3: Commit (if build artifacts should be tracked, otherwise add to .gitignore)**

Since WASM files are build artifacts, add to .gitignore:

```bash
echo "/target/" >> .gitignore
git add .gitignore
git commit -m "chore: add target/ to gitignore"
```

---

### Task 7: Verify Extension Structure

**Step 1: List all extension files**

Run: `find . -type f -not -path './.git/*' -not -path './target/*' | sort`

Expected structure:
```
./.gitignore
./Cargo.toml
./extension.toml
./languages/pstheme/config.toml
./languages/pstheme/highlights.scm
./src/lib.rs
./test/fixtures/minimal.pstheme
./test/fixtures/valid.pstheme
```

**Step 2: Final commit**

```bash
git add -A
git commit -m "feat: complete Zed extension for PaletteSwap"
```

---

## Installation Instructions (for manual testing)

1. Build the extension: `cargo build --release --target wasm32-wasi`
2. In Zed: Open Command Palette → `zed: install dev extension`
3. Select the extension directory (where extension.toml is located)
4. Open a `.pstheme` file to test syntax highlighting and LSP features

---

## Troubleshooting

- **Build fails:** Ensure Rust is installed via rustup, not homebrew
- **LSP not working:** Verify `pstheme-lsp` is in PATH: `which pstheme-lsp`
- **No syntax highlighting:** Check that `languages/pstheme/config.toml` has correct `grammar = "hcl"`

---

## References

- Zed Extension Docs: https://zed.dev/docs/extensions/developing-extensions
- Zed Language Extensions: https://zed.dev/docs/extensions/languages
- Tree-sitter HCL Grammar: https://github.com/tree-sitter-grammars/tree-sitter-hcl
- Zed Extension API: https://docs.rs/zed_extension_api
