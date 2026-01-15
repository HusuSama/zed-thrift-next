//! zed-thrift-next extension
//! The extension is a separate project from zed-thrift and has no association or dependency with it.
//! The zed-thrift project has some implementation issues, which makes the extension difficult to use fully.
//! Therefore, zed-thrift-next was created to improve functionality and automatically install the binary program.
use std::{fs, path::Path};

use zed_extension_api::{self as zed, GithubReleaseOptions, Result};

const LSPOWNER: &str = "joyme123";
const LSPREPO: &str = "thrift-ls";
const FALLBACK_NAME: &str = "thriftls";
const LANGUAGE_SERVER_NAME: &str = "Thrift";

/// thrift extension, this extension will automatically download and install the thriftls binary
#[derive(Default)]
struct ThriftNext {
    // The name of the thriftls that needs to be downloaded
    binary_name: String,
    // Binary file storage location, subsequent command executions will use this address
    // If the binary file is already installed and is in $PATH, this path will only contain the binary name
    // If installed but not in $PATH, it will look up the latest version on GitHub, check if the latest version is installed, and assign this variable to the download path
    cache_binary_path: String,
}

impl zed::Extension for ThriftNext {
    fn new() -> Self
    where
        Self: Sized,
    {
        let mut thrift_next = Self::default();
        // Initialize at startup to get the platform binary name, if it fails, use thriftls
        thrift_next.binary_name = thrift_next
            .get_binary_name()
            .unwrap_or(FALLBACK_NAME.to_string());
        thrift_next
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> Result<zed_extension_api::Command> {
        let env = _worktree.shell_env();

        let mut binary_path = String::new();
        let mut args = Vec::new();

        // Get user LSP configuration
        // If the user has explicitly configured binary and arguments, the extension will no longer check for updates or perform other configurations
        if let Ok(lsp_config) =
            zed::settings::LspSettings::for_worktree(LANGUAGE_SERVER_NAME, _worktree)
        {
            if let Some(binary) = lsp_config.binary {
                if let Some(path) = binary.path {
                    binary_path = path;
                } else {
                    binary_path =
                        self.install_thrift_from_release(_worktree, _language_server_id)?;
                }
                if let Some(arguments) = binary.arguments {
                    args = arguments;
                }
            } else {
                binary_path = self.install_thrift_from_release(_worktree, _language_server_id)?;
            }
        }

        Ok(zed::Command {
            command: binary_path,
            args,
            env,
        })
    }
}

impl ThriftNext {
    /// Get the binary name, used for searching releases and finding the current PATH
    fn get_binary_name(&self) -> Result<String> {
        let (platform, arch) = zed::current_platform();
        Ok(match platform {
            zed::Os::Mac => match arch {
                zed::Architecture::Aarch64 => "thriftls-darwin-arm64",
                zed::Architecture::X8664 => "thriftls-darwin-amd64",
                _ => return Err("Unsupported macOS architecture".to_string()),
            },
            zed::Os::Linux => match arch {
                zed::Architecture::Aarch64 => "thriftls-linux-arm64",
                zed::Architecture::X8664 => "thriftls-linux-amd64",
                zed::Architecture::X86 => "thriftls-linux-386",
                _ => return Err("Unsupported Linux architecture".to_string()),
            },
            zed::Os::Windows => match arch {
                zed::Architecture::X8664 => "thriftls-windows-amd64.exe",
                zed::Architecture::X86 => "thriftls-windows-386.exe",
                _ => return Err("Unsupported Windows architecture".to_string()),
            },
        }
        .to_string())
    }

    /// Check if thriftls already exists on the current system. If it does, assign the existing binary name.
    fn has_thriftls_in_env(&mut self, worktree: &zed::Worktree) -> bool {
        // If the thriftls binary file exists in $PATH, it will not be downloaded
        if worktree.which(&self.binary_name).is_some() {
            // If it exists, assign the original binary name to cache_binary_path
            self.cache_binary_path = self.binary_name.clone();
            return true;
        } else if worktree.which(FALLBACK_NAME).is_some() {
            // Use FALLBACK_NAME to search again, compatible with user renaming operations
            // If found, assign the value to cache_binary_path
            self.cache_binary_path = FALLBACK_NAME.to_string();
            return true;
        }
        false
    }

    /// Install the thriftls binary and return the execution path after installation
    /// When the binary file exists in $PATH, the returned value is the binary file name.
    /// When the binary file is not in $PATH, but there is a download directory, the returned value is the current binary path.
    /// If none exist, return the downloaded binary file path.
    fn install_thrift_from_release(
        &mut self,
        worktree: &zed::Worktree,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        if self.has_thriftls_in_env(worktree) {
            return Ok(self.binary_name.clone());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let repo = format!("{}/{}", LSPOWNER, LSPREPO);
        let release = zed::latest_github_release(
            &repo,
            GithubReleaseOptions {
                pre_release: false,
                require_assets: true,
            },
        )?;
        let version_dir = format!("bin/{}", release.version);
        let binary_path = format!("{}/{}", version_dir, self.binary_name);
        // If this path already exists, it means it has been downloaded before, and the download task will not be executed again
        if Path::new(&binary_path).try_exists().is_ok_and(|x| x) {
            zed::make_file_executable(&binary_path)?;
            self.cache_binary_path = binary_path.clone();
            self.cleanup_old_versions(&release.version);
            return Ok(binary_path);
        };

        // Download from Github release
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == self.binary_name)
            .ok_or_else(|| {
                format!(
                    "No asset found matching {:?} in release {}",
                    self.binary_name, release.version
                )
            })?;

        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("Create version directory failed: {}", e))?;

        zed::download_file(
            &asset.download_url,
            &binary_path,
            zed::DownloadedFileType::Uncompressed,
        )
        .map_err(|e| format!("Failed to download file: {}", e))?;
        zed::make_file_executable(&binary_path)?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        // Clean up old versions after successful download
        self.cleanup_old_versions(&release.version);
        self.cache_binary_path = binary_path.clone();

        Ok(binary_path)
    }

    /// Clean up old versions based on folder names, downloads will keep the form bin/<version>/<binary_name>
    fn cleanup_old_versions(&self, current_version: &str) {
        let Ok(entries) = fs::read_dir("bin") else {
            return;
        };

        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };

            if !file_type.is_dir() {
                continue;
            };

            let file_name = entry.file_name();
            let Some(dir_name) = file_name.to_str() else {
                continue;
            };

            if dir_name != current_version {
                let _ = fs::remove_dir_all(entry.path());
            }
        }
    }
}

zed::register_extension!(ThriftNext);
