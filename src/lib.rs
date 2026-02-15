use zed_extension_api::{self as zed, LanguageServerId, Worktree};

const GITHUB_REPO: &str = "jsvensson/paletteswap";
const BINARY_NAME: &str = "pstheme-lsp";

struct PaletteSwapExtension;

impl PaletteSwapExtension {
    fn get_asset_name(&self) -> zed::Result<String> {
        let (os, arch) = zed::current_platform();

        let os_name = match os {
            zed::Os::Mac => "Darwin",
            zed::Os::Linux => "Linux",
            zed::Os::Windows => "Windows",
        };

        let arch_name = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "x86_64",
            _ => return Err(format!("Unsupported architecture: {:?}", arch)),
        };

        let ext = if os == zed::Os::Windows {
            "zip"
        } else {
            "tar.gz"
        };

        Ok(format!("paletteswap_{}_{}.{}", os_name, arch_name, ext))
    }

    fn get_binary_path(&self) -> zed::Result<String> {
        // Check if binary already exists in extension's working directory
        let binary_path = BINARY_NAME.to_string();

        if std::fs::metadata(&binary_path).is_ok() {
            return Ok(binary_path);
        }

        // Get latest release
        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("Failed to fetch release: {}", e))?;

        // Find the appropriate asset
        let asset_name = self.get_asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("Asset {} not found in release", asset_name))?;

        // Download the asset
        let file_type = if asset_name.ends_with(".zip") {
            zed::DownloadedFileType::Zip
        } else {
            zed::DownloadedFileType::GzipTar
        };

        zed::download_file(&asset.download_url, &asset_name, file_type)
            .map_err(|e| format!("Failed to download asset: {}", e))?;

        // Make it executable on Unix systems
        #[cfg(unix)]
        {
            zed::make_file_executable(&binary_path)
                .map_err(|e| format!("Failed to make binary executable: {}", e))?;
        }

        Ok(binary_path)
    }
}

impl zed::Extension for PaletteSwapExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let binary_path = self.get_binary_path()?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(PaletteSwapExtension);
