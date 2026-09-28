use std::path::Path;
use zed_extension_api::{
    self as zed,
    settings::LspSettings,
    LanguageServerId, Result,
};

struct JacExtension;

impl JacExtension {
    fn language_server_binary(
        &self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1. Check user-defined binary in Zed settings
        if let Ok(lsp_settings) = LspSettings::for_worktree(server_id.as_ref(), worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    return Ok(path);
                }
            }
        }

        // 2. Check virtual environment binaries inside the worktree root
        let root = worktree.root_path();
        let (os, _) = zed::current_platform();
        let (bin_folder, exe_name) = match os {
            zed::Os::Windows => ("Scripts", "jac.exe"),
            _ => ("bin", "jac"),
        };

        let venv_dirs = [".venv", "venv", "env", ".env"];
        for venv in &venv_dirs {
            let pyvenv_cfg = format!("{venv}/pyvenv.cfg");
            if worktree.read_text_file(&pyvenv_cfg).is_ok() {
                let venv_jac = format!("{root}/{venv}/{bin_folder}/{exe_name}");
                return Ok(venv_jac);
            }
        }

        // 3. Search for jac on the user's PATH via the worktree
        if let Some(path) = worktree.which(exe_name) {
            return Ok(path);
        }

        // 4. Check common installation locations
        if let Some(home) = worktree
            .shell_env()
            .iter()
            .find(|(k, _)| k == "HOME")
            .map(|(_, v)| v)
        {
            let user_local_jac = format!("{home}/.local/bin/{exe_name}");
            if Path::new(&user_local_jac).exists() {
                return Ok(user_local_jac);
            }
        }

        let fallback_paths = [
            "/opt/homebrew/bin/jac",
            "/usr/local/bin/jac",
            "/usr/bin/jac",
        ];
        for candidate in fallback_paths {
            if Path::new(candidate).exists() {
                return Ok(candidate.to_string());
            }
        }

        // 5. Final fallback to executable name in PATH
        Ok(exe_name.to_string())
    }
}

impl zed::Extension for JacExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        let mut args = vec!["lsp".to_string()];
        if let Ok(lsp_settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(custom_args) = binary.arguments {
                    if !custom_args.is_empty() {
                        args = custom_args;
                    }
                }
            }
        }

        Ok(zed::Command {
            command: binary,
            args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let options = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone());
        Ok(options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let config = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone());
        Ok(config)
    }
}

zed::register_extension!(JacExtension);
