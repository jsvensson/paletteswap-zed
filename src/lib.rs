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
