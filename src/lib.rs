use zed_extension_api::{self as zed, LanguageServerId, Worktree};

struct PaletteSwapExtension;

impl zed::Extension for PaletteSwapExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "pstheme-lsp".to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(PaletteSwapExtension);
