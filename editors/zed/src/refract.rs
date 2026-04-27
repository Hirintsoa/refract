use zed_extension_api::{self as zed, LanguageServerId, Result};

struct RefractExtension;

impl zed::Extension for RefractExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("refract")
            .ok_or_else(|| "refract binary not found on PATH — install from https://github.com/hrtsx/refract".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(zed::serde_json::json!({
            "disableGemIndex": false,
            "disableRubocop": false,
            "logLevel": 2,
        })))
    }
}

zed::register_extension!(RefractExtension);
