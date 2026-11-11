//! Translations for the strings Rust renders natively (tray menu, window
//! titles, OS notifications, the HUD, file-dialog filters) or hands to the
//! frontend fully formed (a few command results, and the user-facing error
//! messages built in the core: AI clients, the agent runner, the script
//! scanner, MCP server tests, extension installs). Everything else is localised in the frontend catalogs under
//! `src/locales/`; keep these tables in sync with the languages shipped there.

use super::ParsedLocale;

/// Languages with a native-string table. Mirrors the frontend catalogs
/// (`en`, `pt-BR`, `zh-CN`, `zh-Hant`, `fr`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeLang {
    En,
    PtBr,
    ZhHans,
    ZhHant,
    Fr,
}

/// Strings Rust shows to the user without going through the frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeText {
    TrayQuit,
    TrayCheckUpdates,
    TraySettings,
    SettingsWindowTitle,
    StickyNoteTitle,
    OnboardingTitle,
    /// HUD after a screen OCR capture that found no text.
    OcrNoText,
    /// HUD after a short OCR capture. Placeholder: `{text}`.
    OcrCopiedPreview,
    /// HUD after a long / multi-line OCR capture.
    OcrCopied,
    /// OS notification title when a run fails.
    RunFailedTitle,
    /// OS notification title when a shell script finishes.
    ScriptFinishedTitle,
    /// Notification action button that opens the Runs view.
    RunActionOpen,
    /// Notification action button that shows a script's output.
    RunActionViewOutput,
    /// Notification body fallback when a run produced no output.
    RunNoOutput,
    /// Close button of a macOS notification with actions.
    NotificationClose,
    /// File-dialog filter name for `.asyar` profile files.
    DialogFilterProfile,
    /// File-dialog filter name for `.asyar` extension packages.
    DialogFilterPackage,
    /// Settings → AI: why a provider can't be removed.
    /// Placeholders: `{provider_name}`, `{agent_names}`.
    ProviderRemovalBlocked,
    /// Settings → AI: CLI binary missing. Placeholder: `{name}`.
    CliBinaryNotFound,
    /// AI extension builder: no Anthropic key saved.
    ExtBuilderApiKeyMissing,
    /// AI extension builder: the saved Anthropic key can't be decrypted.
    ExtBuilderApiKeyUndecryptable,
    /// Settings → AI: the CLI's `--version` check exited non-zero.
    CliVersionNonZero,
    /// Settings → AI: the CLI binary could not be run. Placeholder: `{error}`.
    CliExecFailed,
    /// Settings → AI: the CLI's `--version` check timed out.
    CliVersionTimeout,
    /// AI chat: Claude Code failed without a message.
    ClaudeCodeReportedError,
    /// AI chat: the CLI process failed. Placeholder: `{status}`.
    CliExitedWithCode,
    /// AI chat: no CLI binary for the engine. Placeholder: `{engine}`.
    CliEngineBinaryNotFound,
    /// AI chat: the CLI process could not start. Placeholders: `{path}`, `{error}`.
    CliSpawnFailed,
    /// Codex: the user's home directory is unknown.
    CodexHomeUnresolved,
    /// Codex: no `auth.json` login to link.
    CodexLoginNotFound,
    /// Codex: linking the login failed. Placeholder: `{error}`.
    CodexLinkFailed,
    /// Codex: `codex app-server` could not start. Placeholders: `{path}`, `{error}`.
    CodexSpawnFailed,
    /// Codex: listing models timed out.
    CodexModelsTimeout,
    /// Codex: `thread/start` failed without a message.
    CodexThreadStartFailed,
    /// Codex: `thread/start` returned no thread id.
    CodexThreadIdMissing,
    /// Codex: reading the stream failed. Placeholder: `{error}`.
    CodexStreamReadError,
    /// Codex: an `error` notification without a message.
    CodexUnknownError,
    /// Codex: `turn/start` failed without a message.
    CodexTurnStartFailed,
    /// AI chat: OpenAI Responses stream failed without a message.
    OpenAiResponsesRequestFailed,
    /// AI chat: Anthropic stream failed without a message.
    AnthropicRequestFailed,
    /// AI chat: Google Search + agent tools on a pre-Gemini 3 model.
    GoogleSearchNeedsGemini3,
    /// AI chat: custom provider without a base URL.
    CustomProviderBaseUrlEmpty,
    /// Settings → AI: listing models of a custom provider without a base URL.
    CustomProviderRequiresBaseUrl,
    /// Settings → AI: saving an empty API key.
    CredentialSecretEmpty,
    /// Agent run: provider missing. Placeholder: `{provider}`.
    ProviderNotConfigured,
    /// Agent run: provider disabled. Placeholder: `{provider}`.
    ProviderDisabled,
    /// Agent run: unknown provider engine. Placeholder: `{provider}`.
    ProviderNotRegistered,
    /// Agent run: CLI mode on a non-CLI provider. Placeholder: `{provider}`.
    ProviderCliModeUnsupported,
    /// Agent run: CLI binary missing. Placeholder: `{provider}`.
    ProviderCliNotFound,
    /// AI: stored key can't be decrypted. Placeholder: `{provider}`.
    ProviderApiKeyUndecryptable,
    /// Settings → AI: stored key can't be decrypted. Placeholder: `{provider}`.
    ProviderApiKeyUndecryptableShort,
    /// Agent run: no API key saved. Placeholder: `{provider}`.
    ProviderApiKeyMissing,
    /// Agent run: no base URL saved. Placeholder: `{provider}`.
    ProviderBaseUrlMissing,
    /// Agent run: an extension AI provider failed. Placeholder: `{error}`.
    ExtensionAiProviderFailed,
    /// Script library issue. Placeholder: `{error}`.
    ScriptDirUnreadable,
    /// Script library issue. Placeholder: `{error}`.
    ScriptMetadataUnreadable,
    /// Script library issue. Placeholder: `{error}`.
    ScriptPathUnavailable,
    /// Script library issue: missing execute bit.
    ScriptNotExecutable,
    /// Script library issue. Placeholder: `{error}`.
    ScriptContentUnreadable,
    /// Script library issue. Placeholder: `{error}`.
    ScriptInvalidHeader,
    /// Script header error. Placeholders: `{line}`, `{message}`.
    ScriptHeaderArgumentJson,
    /// Script header error. Placeholder: `{index}`.
    ScriptHeaderArgumentIndex,
    /// Script header error. Placeholder: `{index}`.
    ScriptHeaderDuplicateIndex,
    /// Script header error. Placeholder: `{value}`.
    ScriptHeaderInvalidMode,
    /// Script header error. Placeholders: `{line}`, `{message}`.
    ScriptHeaderRequireAnyOf,
    /// Script header error. Placeholder: `{value}`.
    ScriptHeaderRefreshTime,
    /// MCP server test. Placeholder: `{error}`.
    McpTransportError,
    /// MCP server test. Placeholder: `{error}`.
    McpProtocolError,
    /// MCP server test. Placeholders: `{code}`, `{message}`.
    McpRpcError,
    /// MCP server test: the server quit during the handshake.
    McpEarlyExit,
    /// MCP server test. Placeholder: `{id}`.
    McpTimeout,
    /// Extension install. Placeholders: `{name}`, `{platform}`, `{supported}`.
    ExtPlatformUnsupported,
    /// Extension install. Placeholders: `{name}`, `{platform}`, `{supported}`.
    ExtPlatformUnsupportedDeclared,
    /// Extension install: empty supported-platform list.
    ExtSupportedNone,
    /// Extension install: empty supported-platform list.
    ExtSupportedNoneDeclared,
    /// Extension install. Placeholders: `{id}`, `{version}`.
    ExtAlreadyInstalled,
    /// Extension install from a file that isn't `.asyar`.
    ExtFileMustBeAsyar,
    /// Extension install. Placeholders: `{expected}`, `{calculated}`.
    ExtChecksumMismatch,
    /// Extension install. Placeholder: `{url}`.
    ExtHttpsRequired,
}

impl NativeText {
    /// Every key, for exhaustiveness tests.
    #[cfg(test)]
    pub const ALL: &'static [NativeText] = &[
        NativeText::TrayQuit,
        NativeText::TrayCheckUpdates,
        NativeText::TraySettings,
        NativeText::SettingsWindowTitle,
        NativeText::StickyNoteTitle,
        NativeText::OnboardingTitle,
        NativeText::OcrNoText,
        NativeText::OcrCopiedPreview,
        NativeText::OcrCopied,
        NativeText::RunFailedTitle,
        NativeText::ScriptFinishedTitle,
        NativeText::RunActionOpen,
        NativeText::RunActionViewOutput,
        NativeText::RunNoOutput,
        NativeText::NotificationClose,
        NativeText::DialogFilterProfile,
        NativeText::DialogFilterPackage,
        NativeText::ProviderRemovalBlocked,
        NativeText::CliBinaryNotFound,
        NativeText::ExtBuilderApiKeyMissing,
        NativeText::ExtBuilderApiKeyUndecryptable,
        NativeText::CliVersionNonZero,
        NativeText::CliExecFailed,
        NativeText::CliVersionTimeout,
        NativeText::ClaudeCodeReportedError,
        NativeText::CliExitedWithCode,
        NativeText::CliEngineBinaryNotFound,
        NativeText::CliSpawnFailed,
        NativeText::CodexHomeUnresolved,
        NativeText::CodexLoginNotFound,
        NativeText::CodexLinkFailed,
        NativeText::CodexSpawnFailed,
        NativeText::CodexModelsTimeout,
        NativeText::CodexThreadStartFailed,
        NativeText::CodexThreadIdMissing,
        NativeText::CodexStreamReadError,
        NativeText::CodexUnknownError,
        NativeText::CodexTurnStartFailed,
        NativeText::OpenAiResponsesRequestFailed,
        NativeText::AnthropicRequestFailed,
        NativeText::GoogleSearchNeedsGemini3,
        NativeText::CustomProviderBaseUrlEmpty,
        NativeText::CustomProviderRequiresBaseUrl,
        NativeText::CredentialSecretEmpty,
        NativeText::ProviderNotConfigured,
        NativeText::ProviderDisabled,
        NativeText::ProviderNotRegistered,
        NativeText::ProviderCliModeUnsupported,
        NativeText::ProviderCliNotFound,
        NativeText::ProviderApiKeyUndecryptable,
        NativeText::ProviderApiKeyUndecryptableShort,
        NativeText::ProviderApiKeyMissing,
        NativeText::ProviderBaseUrlMissing,
        NativeText::ExtensionAiProviderFailed,
        NativeText::ScriptDirUnreadable,
        NativeText::ScriptMetadataUnreadable,
        NativeText::ScriptPathUnavailable,
        NativeText::ScriptNotExecutable,
        NativeText::ScriptContentUnreadable,
        NativeText::ScriptInvalidHeader,
        NativeText::ScriptHeaderArgumentJson,
        NativeText::ScriptHeaderArgumentIndex,
        NativeText::ScriptHeaderDuplicateIndex,
        NativeText::ScriptHeaderInvalidMode,
        NativeText::ScriptHeaderRequireAnyOf,
        NativeText::ScriptHeaderRefreshTime,
        NativeText::McpTransportError,
        NativeText::McpProtocolError,
        NativeText::McpRpcError,
        NativeText::McpEarlyExit,
        NativeText::McpTimeout,
        NativeText::ExtPlatformUnsupported,
        NativeText::ExtPlatformUnsupportedDeclared,
        NativeText::ExtSupportedNone,
        NativeText::ExtSupportedNoneDeclared,
        NativeText::ExtAlreadyInstalled,
        NativeText::ExtFileMustBeAsyar,
        NativeText::ExtChecksumMismatch,
        NativeText::ExtHttpsRequired,
    ];
}

impl NativeLang {
    /// Picks the table for a locale. Chinese is script-split: an explicit script
    /// wins, otherwise the Traditional regions (TW/HK/MO) map to `ZhHant`, and
    /// everything else to `ZhHans`. Unknown languages fall back to English.
    pub fn from_locale(locale: &ParsedLocale) -> Self {
        match locale.language.as_str() {
            "pt" => Self::PtBr,
            "fr" => Self::Fr,
            "zh" => match (locale.script.as_deref(), locale.region.as_deref()) {
                (Some("Hant"), _) => Self::ZhHant,
                (Some(_), _) => Self::ZhHans,
                (None, Some("TW" | "HK" | "MO")) => Self::ZhHant,
                _ => Self::ZhHans,
            },
            _ => Self::En,
        }
    }
}

/// Returns the translated string for `key` in the locale's language.
pub fn native_text(locale: &ParsedLocale, key: NativeText) -> &'static str {
    let lang = NativeLang::from_locale(locale);
    match (key, lang) {
        (NativeText::TrayQuit, NativeLang::En) => "Quit Asyar",
        (NativeText::TrayQuit, NativeLang::PtBr) => "Sair do Asyar",
        (NativeText::TrayQuit, NativeLang::ZhHans) => "退出 Asyar",
        (NativeText::TrayQuit, NativeLang::ZhHant) => "結束 Asyar",
        (NativeText::TrayQuit, NativeLang::Fr) => "Quitter Asyar",

        (NativeText::TrayCheckUpdates, NativeLang::En) => "Check for Updates",
        (NativeText::TrayCheckUpdates, NativeLang::PtBr) => "Verificar atualizações",
        (NativeText::TrayCheckUpdates, NativeLang::ZhHans) => "检查更新",
        (NativeText::TrayCheckUpdates, NativeLang::ZhHant) => "檢查更新",
        (NativeText::TrayCheckUpdates, NativeLang::Fr) => "Rechercher des mises à jour",

        (NativeText::TraySettings, NativeLang::En) => "Settings",
        (NativeText::TraySettings, NativeLang::PtBr) => "Configurações",
        (NativeText::TraySettings, NativeLang::ZhHans) => "设置",
        (NativeText::TraySettings, NativeLang::ZhHant) => "設定",
        (NativeText::TraySettings, NativeLang::Fr) => "Réglages",

        (NativeText::SettingsWindowTitle, NativeLang::En) => "Asyar Settings",
        (NativeText::SettingsWindowTitle, NativeLang::PtBr) => "Configurações do Asyar",
        (NativeText::SettingsWindowTitle, NativeLang::ZhHans) => "Asyar 设置",
        (NativeText::SettingsWindowTitle, NativeLang::ZhHant) => "Asyar 設定",
        (NativeText::SettingsWindowTitle, NativeLang::Fr) => "Réglages Asyar",

        (NativeText::StickyNoteTitle, NativeLang::En) => "Sticky Note",
        (NativeText::StickyNoteTitle, NativeLang::PtBr) => "Nota adesiva",
        (NativeText::StickyNoteTitle, NativeLang::ZhHans) => "便笺",
        (NativeText::StickyNoteTitle, NativeLang::ZhHant) => "便利貼",
        (NativeText::StickyNoteTitle, NativeLang::Fr) => "Pense-bête",

        (NativeText::OnboardingTitle, NativeLang::En) => "Welcome to Asyar",
        (NativeText::OnboardingTitle, NativeLang::PtBr) => "Boas-vindas ao Asyar",
        (NativeText::OnboardingTitle, NativeLang::ZhHans) => "欢迎使用 Asyar",
        (NativeText::OnboardingTitle, NativeLang::ZhHant) => "歡迎使用 Asyar",
        (NativeText::OnboardingTitle, NativeLang::Fr) => "Bienvenue dans Asyar",

        (NativeText::OcrNoText, NativeLang::En) => "No text found in selected area",
        (NativeText::OcrNoText, NativeLang::PtBr) => "Nenhum texto encontrado na área selecionada",
        (NativeText::OcrNoText, NativeLang::ZhHans) => "所选区域中未找到文本",
        (NativeText::OcrNoText, NativeLang::ZhHant) => "所選區域中找不到文字",
        (NativeText::OcrNoText, NativeLang::Fr) => "Aucun texte trouvé dans la zone sélectionnée",

        (NativeText::OcrCopiedPreview, NativeLang::En) => "Text copied: \"{text}\"",
        (NativeText::OcrCopiedPreview, NativeLang::PtBr) => "Texto copiado: \"{text}\"",
        (NativeText::OcrCopiedPreview, NativeLang::ZhHans) => "已复制文本：“{text}”",
        (NativeText::OcrCopiedPreview, NativeLang::ZhHant) => "已複製文字：「{text}」",
        (NativeText::OcrCopiedPreview, NativeLang::Fr) => "Texte copié : « {text} »",

        (NativeText::OcrCopied, NativeLang::En) => "Text copied to clipboard",
        (NativeText::OcrCopied, NativeLang::PtBr) => "Texto copiado para a área de transferência",
        (NativeText::OcrCopied, NativeLang::ZhHans) => "文本已复制到剪贴板",
        (NativeText::OcrCopied, NativeLang::ZhHant) => "文字已複製到剪貼簿",
        (NativeText::OcrCopied, NativeLang::Fr) => "Texte copié dans le presse-papiers",

        (NativeText::RunFailedTitle, NativeLang::En) => "Run failed",
        (NativeText::RunFailedTitle, NativeLang::PtBr) => "Falha na execução",
        (NativeText::RunFailedTitle, NativeLang::ZhHans) => "运行失败",
        (NativeText::RunFailedTitle, NativeLang::ZhHant) => "執行失敗",
        (NativeText::RunFailedTitle, NativeLang::Fr) => "Échec de l'exécution",

        (NativeText::ScriptFinishedTitle, NativeLang::En) => "Script finished",
        (NativeText::ScriptFinishedTitle, NativeLang::PtBr) => "Script concluído",
        (NativeText::ScriptFinishedTitle, NativeLang::ZhHans) => "脚本已完成",
        (NativeText::ScriptFinishedTitle, NativeLang::ZhHant) => "指令碼已完成",
        (NativeText::ScriptFinishedTitle, NativeLang::Fr) => "Script terminé",

        (NativeText::RunActionOpen, NativeLang::En) => "Open",
        (NativeText::RunActionOpen, NativeLang::PtBr) => "Abrir",
        (NativeText::RunActionOpen, NativeLang::ZhHans) => "打开",
        (NativeText::RunActionOpen, NativeLang::ZhHant) => "開啟",
        (NativeText::RunActionOpen, NativeLang::Fr) => "Ouvrir",

        (NativeText::RunActionViewOutput, NativeLang::En) => "View output",
        (NativeText::RunActionViewOutput, NativeLang::PtBr) => "Ver saída",
        (NativeText::RunActionViewOutput, NativeLang::ZhHans) => "查看输出",
        (NativeText::RunActionViewOutput, NativeLang::ZhHant) => "檢視輸出",
        (NativeText::RunActionViewOutput, NativeLang::Fr) => "Voir le résultat",

        (NativeText::RunNoOutput, NativeLang::En) => "(no output)",
        (NativeText::RunNoOutput, NativeLang::PtBr) => "(sem saída)",
        (NativeText::RunNoOutput, NativeLang::ZhHans) => "（无输出）",
        (NativeText::RunNoOutput, NativeLang::ZhHant) => "（無輸出）",
        (NativeText::RunNoOutput, NativeLang::Fr) => "(aucun résultat)",

        (NativeText::NotificationClose, NativeLang::En) => "Close",
        (NativeText::NotificationClose, NativeLang::PtBr) => "Fechar",
        (NativeText::NotificationClose, NativeLang::ZhHans) => "关闭",
        (NativeText::NotificationClose, NativeLang::ZhHant) => "關閉",
        (NativeText::NotificationClose, NativeLang::Fr) => "Fermer",

        (NativeText::DialogFilterProfile, NativeLang::En) => "Asyar Profile",
        (NativeText::DialogFilterProfile, NativeLang::PtBr) => "Perfil do Asyar",
        (NativeText::DialogFilterProfile, NativeLang::ZhHans) => "Asyar 配置文件",
        (NativeText::DialogFilterProfile, NativeLang::ZhHant) => "Asyar 設定檔",
        (NativeText::DialogFilterProfile, NativeLang::Fr) => "Profil Asyar",

        (NativeText::DialogFilterPackage, NativeLang::En) => "Asyar Package",
        (NativeText::DialogFilterPackage, NativeLang::PtBr) => "Pacote do Asyar",
        (NativeText::DialogFilterPackage, NativeLang::ZhHans) => "Asyar 扩展包",
        (NativeText::DialogFilterPackage, NativeLang::ZhHant) => "Asyar 擴充套件包",
        (NativeText::DialogFilterPackage, NativeLang::Fr) => "Paquet Asyar",

        (NativeText::ProviderRemovalBlocked, NativeLang::En) => {
            "Can't remove {provider_name} — it's the last configured provider and these agents \
             still use it: {agent_names}. Reassign or delete them first."
        }
        (NativeText::ProviderRemovalBlocked, NativeLang::PtBr) => {
            "Não é possível remover {provider_name} — é o último provedor configurado e estes \
             agentes ainda o usam: {agent_names}. Reatribua-os ou exclua-os primeiro."
        }
        (NativeText::ProviderRemovalBlocked, NativeLang::ZhHans) => {
            "无法移除 {provider_name}——它是最后一个已配置的提供商，以下智能体仍在使用它：\
             {agent_names}。请先重新分配或删除它们。"
        }
        (NativeText::ProviderRemovalBlocked, NativeLang::ZhHant) => {
            "無法移除 {provider_name}——它是最後一個已設定的提供者，以下代理仍在使用它：\
             {agent_names}。請先重新指派或刪除它們。"
        }
        (NativeText::ProviderRemovalBlocked, NativeLang::Fr) => {
            "Impossible de supprimer {provider_name} : c'est le dernier fournisseur configuré et \
             ces agents l'utilisent encore : {agent_names}. Réattribuez-les ou supprimez-les d'abord."
        }

        (NativeText::CliBinaryNotFound, NativeLang::En) => {
            "{name} CLI binary not found on this machine."
        }
        (NativeText::CliBinaryNotFound, NativeLang::PtBr) => {
            "Binário da CLI {name} não encontrado neste computador."
        }
        (NativeText::CliBinaryNotFound, NativeLang::ZhHans) => "在此设备上未找到 {name} CLI 可执行文件。",
        (NativeText::CliBinaryNotFound, NativeLang::ZhHant) => "在此裝置上找不到 {name} CLI 執行檔。",
        (NativeText::CliBinaryNotFound, NativeLang::Fr) => {
            "Binaire CLI {name} introuvable sur cet ordinateur."
        }

        (NativeText::ExtBuilderApiKeyMissing, NativeLang::En) => {
            "Anthropic API key is not configured. Please set your API key in Settings → AI."
        }
        (NativeText::ExtBuilderApiKeyMissing, NativeLang::PtBr) => {
            "A chave de API da Anthropic não está configurada. Defina sua chave de API em \
             Configurações → IA."
        }
        (NativeText::ExtBuilderApiKeyMissing, NativeLang::ZhHans) => {
            "尚未配置 Anthropic API 密钥。请在“设置 → AI”中设置你的 API 密钥。"
        }
        (NativeText::ExtBuilderApiKeyMissing, NativeLang::ZhHant) => {
            "尚未設定 Anthropic API 金鑰。請在「設定 → AI」中設定你的 API 金鑰。"
        }
        (NativeText::ExtBuilderApiKeyMissing, NativeLang::Fr) => {
            "Aucune clé API Anthropic n'est configurée. Définissez votre clé API dans Réglages → IA."
        }

        (NativeText::ExtBuilderApiKeyUndecryptable, NativeLang::En) => {
            "API key for Anthropic cannot be decrypted (keychain master key changed). Please \
             re-enter your key in Settings → AI."
        }
        (NativeText::ExtBuilderApiKeyUndecryptable, NativeLang::PtBr) => {
            "Não foi possível descriptografar a chave de API da Anthropic (a chave mestra do \
             chaveiro mudou). Insira sua chave novamente em Configurações → IA."
        }
        (NativeText::ExtBuilderApiKeyUndecryptable, NativeLang::ZhHans) => {
            "无法解密 Anthropic 的 API 密钥（钥匙串主密钥已更改）。请在“设置 → AI”中重新输入密钥。"
        }
        (NativeText::ExtBuilderApiKeyUndecryptable, NativeLang::ZhHant) => {
            "無法解密 Anthropic 的 API 金鑰（鑰匙圈主金鑰已變更）。請在「設定 → AI」中重新輸入金鑰。"
        }
        (NativeText::ExtBuilderApiKeyUndecryptable, NativeLang::Fr) => {
            "Impossible de déchiffrer la clé API Anthropic (la clé maîtresse du trousseau a changé). \
             Saisissez à nouveau votre clé dans Réglages → IA."
        }

        (NativeText::CliVersionNonZero, NativeLang::En) => "CLI exited with non-zero status on version check",
        (NativeText::CliVersionNonZero, NativeLang::PtBr) => "A CLI terminou com status diferente de zero ao verificar a versão",
        (NativeText::CliVersionNonZero, NativeLang::ZhHans) => "检查版本时 CLI 以非零状态退出",
        (NativeText::CliVersionNonZero, NativeLang::ZhHant) => "檢查版本時 CLI 以非零狀態結束",
        (NativeText::CliVersionNonZero, NativeLang::Fr) => "La CLI s'est terminée avec une erreur lors de la vérification de version",

        (NativeText::CliExecFailed, NativeLang::En) => "Failed to execute CLI binary: {error}",
        (NativeText::CliExecFailed, NativeLang::PtBr) => "Falha ao executar o binário da CLI: {error}",
        (NativeText::CliExecFailed, NativeLang::ZhHans) => "无法执行 CLI 可执行文件：{error}",
        (NativeText::CliExecFailed, NativeLang::ZhHant) => "無法執行 CLI 執行檔：{error}",
        (NativeText::CliExecFailed, NativeLang::Fr) => "Impossible d'exécuter le binaire CLI : {error}",

        (NativeText::CliVersionTimeout, NativeLang::En) => "Timeout querying CLI version",
        (NativeText::CliVersionTimeout, NativeLang::PtBr) => "Tempo esgotado ao consultar a versão da CLI",
        (NativeText::CliVersionTimeout, NativeLang::ZhHans) => "查询 CLI 版本超时",
        (NativeText::CliVersionTimeout, NativeLang::ZhHant) => "查詢 CLI 版本逾時",
        (NativeText::CliVersionTimeout, NativeLang::Fr) => "Délai dépassé lors de la lecture de la version de la CLI",

        (NativeText::ClaudeCodeReportedError, NativeLang::En) => "Claude Code reported an error",
        (NativeText::ClaudeCodeReportedError, NativeLang::PtBr) => "O Claude Code relatou um erro",
        (NativeText::ClaudeCodeReportedError, NativeLang::ZhHans) => "Claude Code 报告了一个错误",
        (NativeText::ClaudeCodeReportedError, NativeLang::ZhHant) => "Claude Code 回報了一個錯誤",
        (NativeText::ClaudeCodeReportedError, NativeLang::Fr) => "Claude Code a signalé une erreur",

        (NativeText::CliExitedWithCode, NativeLang::En) => "CLI process exited with code {status}",
        (NativeText::CliExitedWithCode, NativeLang::PtBr) => "O processo da CLI terminou com o código {status}",
        (NativeText::CliExitedWithCode, NativeLang::ZhHans) => "CLI 进程已退出，代码 {status}",
        (NativeText::CliExitedWithCode, NativeLang::ZhHant) => "CLI 處理程序已結束，代碼 {status}",
        (NativeText::CliExitedWithCode, NativeLang::Fr) => "Le processus CLI s'est arrêté avec le code {status}",

        (NativeText::CliEngineBinaryNotFound, NativeLang::En) => "CLI binary for '{engine}' could not be found.",
        (NativeText::CliEngineBinaryNotFound, NativeLang::PtBr) => "Não foi possível encontrar o binário da CLI para '{engine}'.",
        (NativeText::CliEngineBinaryNotFound, NativeLang::ZhHans) => "找不到“{engine}”的 CLI 可执行文件。",
        (NativeText::CliEngineBinaryNotFound, NativeLang::ZhHant) => "找不到「{engine}」的 CLI 執行檔。",
        (NativeText::CliEngineBinaryNotFound, NativeLang::Fr) => "Binaire CLI introuvable pour « {engine} ».",

        (NativeText::CliSpawnFailed, NativeLang::En) => "Failed to spawn CLI process '{path}': {error}",
        (NativeText::CliSpawnFailed, NativeLang::PtBr) => "Falha ao iniciar o processo da CLI '{path}': {error}",
        (NativeText::CliSpawnFailed, NativeLang::ZhHans) => "无法启动 CLI 进程 '{path}'：{error}",
        (NativeText::CliSpawnFailed, NativeLang::ZhHant) => "無法啟動 CLI 處理程序 '{path}'：{error}",
        (NativeText::CliSpawnFailed, NativeLang::Fr) => "Impossible de lancer le processus CLI '{path}' : {error}",

        (NativeText::CodexHomeUnresolved, NativeLang::En) => "Could not resolve user home directory for Codex login",
        (NativeText::CodexHomeUnresolved, NativeLang::PtBr) => "Não foi possível localizar a pasta pessoal do usuário para o login do Codex",
        (NativeText::CodexHomeUnresolved, NativeLang::ZhHans) => "无法确定用于 Codex 登录的用户主目录",
        (NativeText::CodexHomeUnresolved, NativeLang::ZhHant) => "無法確定用於 Codex 登入的使用者主目錄",
        (NativeText::CodexHomeUnresolved, NativeLang::Fr) => "Impossible de trouver le dossier personnel pour la connexion Codex",

        (NativeText::CodexLoginNotFound, NativeLang::En) => "Codex login not found. Please log in using the Codex CLI first.",
        (NativeText::CodexLoginNotFound, NativeLang::PtBr) => "Login do Codex não encontrado. Faça login primeiro usando a CLI do Codex.",
        (NativeText::CodexLoginNotFound, NativeLang::ZhHans) => "未找到 Codex 登录信息。请先使用 Codex CLI 登录。",
        (NativeText::CodexLoginNotFound, NativeLang::ZhHant) => "找不到 Codex 登入資訊。請先使用 Codex CLI 登入。",
        (NativeText::CodexLoginNotFound, NativeLang::Fr) => "Connexion Codex introuvable. Connectez-vous d'abord avec la CLI Codex.",

        (NativeText::CodexLinkFailed, NativeLang::En) => "Couldn't link your Codex login for Asyar: {error}",
        (NativeText::CodexLinkFailed, NativeLang::PtBr) => "Não foi possível vincular seu login do Codex ao Asyar: {error}",
        (NativeText::CodexLinkFailed, NativeLang::ZhHans) => "无法为 Asyar 关联你的 Codex 登录：{error}",
        (NativeText::CodexLinkFailed, NativeLang::ZhHant) => "無法為 Asyar 連結你的 Codex 登入：{error}",
        (NativeText::CodexLinkFailed, NativeLang::Fr) => "Impossible de lier votre connexion Codex à Asyar : {error}",

        (NativeText::CodexSpawnFailed, NativeLang::En) => "Failed to spawn codex app-server '{path}': {error}",
        (NativeText::CodexSpawnFailed, NativeLang::PtBr) => "Falha ao iniciar o codex app-server '{path}': {error}",
        (NativeText::CodexSpawnFailed, NativeLang::ZhHans) => "无法启动 codex app-server '{path}'：{error}",
        (NativeText::CodexSpawnFailed, NativeLang::ZhHant) => "無法啟動 codex app-server '{path}'：{error}",
        (NativeText::CodexSpawnFailed, NativeLang::Fr) => "Impossible de lancer codex app-server '{path}' : {error}",

        (NativeText::CodexModelsTimeout, NativeLang::En) => "Timeout listing models from codex app-server",
        (NativeText::CodexModelsTimeout, NativeLang::PtBr) => "Tempo esgotado ao listar modelos do codex app-server",
        (NativeText::CodexModelsTimeout, NativeLang::ZhHans) => "从 codex app-server 获取模型列表超时",
        (NativeText::CodexModelsTimeout, NativeLang::ZhHant) => "從 codex app-server 取得模型清單逾時",
        (NativeText::CodexModelsTimeout, NativeLang::Fr) => "Délai dépassé lors de la récupération des modèles depuis codex app-server",

        (NativeText::CodexThreadStartFailed, NativeLang::En) => "Failed to start thread in codex app-server",
        (NativeText::CodexThreadStartFailed, NativeLang::PtBr) => "Falha ao iniciar a conversa no codex app-server",
        (NativeText::CodexThreadStartFailed, NativeLang::ZhHans) => "无法在 codex app-server 中启动会话",
        (NativeText::CodexThreadStartFailed, NativeLang::ZhHant) => "無法在 codex app-server 中啟動對話",
        (NativeText::CodexThreadStartFailed, NativeLang::Fr) => "Impossible de démarrer la conversation dans codex app-server",

        (NativeText::CodexThreadIdMissing, NativeLang::En) => "Failed to obtain thread ID from codex app-server",
        (NativeText::CodexThreadIdMissing, NativeLang::PtBr) => "Falha ao obter o ID da conversa do codex app-server",
        (NativeText::CodexThreadIdMissing, NativeLang::ZhHans) => "无法从 codex app-server 获取会话 ID",
        (NativeText::CodexThreadIdMissing, NativeLang::ZhHant) => "無法從 codex app-server 取得對話 ID",
        (NativeText::CodexThreadIdMissing, NativeLang::Fr) => "Impossible d'obtenir l'identifiant de conversation depuis codex app-server",

        (NativeText::CodexStreamReadError, NativeLang::En) => "Stream read error: {error}",
        (NativeText::CodexStreamReadError, NativeLang::PtBr) => "Erro ao ler o fluxo: {error}",
        (NativeText::CodexStreamReadError, NativeLang::ZhHans) => "读取数据流出错：{error}",
        (NativeText::CodexStreamReadError, NativeLang::ZhHant) => "讀取資料流時發生錯誤：{error}",
        (NativeText::CodexStreamReadError, NativeLang::Fr) => "Erreur de lecture du flux : {error}",

        (NativeText::CodexUnknownError, NativeLang::En) => "Unknown error from codex app-server",
        (NativeText::CodexUnknownError, NativeLang::PtBr) => "Erro desconhecido do codex app-server",
        (NativeText::CodexUnknownError, NativeLang::ZhHans) => "codex app-server 发生未知错误",
        (NativeText::CodexUnknownError, NativeLang::ZhHant) => "codex app-server 發生未知錯誤",
        (NativeText::CodexUnknownError, NativeLang::Fr) => "Erreur inconnue de codex app-server",

        (NativeText::CodexTurnStartFailed, NativeLang::En) => "Failed to start turn in codex app-server",
        (NativeText::CodexTurnStartFailed, NativeLang::PtBr) => "Falha ao iniciar a resposta no codex app-server",
        (NativeText::CodexTurnStartFailed, NativeLang::ZhHans) => "无法在 codex app-server 中开始本轮对话",
        (NativeText::CodexTurnStartFailed, NativeLang::ZhHant) => "無法在 codex app-server 中開始本輪對話",
        (NativeText::CodexTurnStartFailed, NativeLang::Fr) => "Impossible de démarrer la réponse dans codex app-server",

        (NativeText::OpenAiResponsesRequestFailed, NativeLang::En) => "OpenAI Responses request failed",
        (NativeText::OpenAiResponsesRequestFailed, NativeLang::PtBr) => "Falha na solicitação ao OpenAI Responses",
        (NativeText::OpenAiResponsesRequestFailed, NativeLang::ZhHans) => "OpenAI Responses 请求失败",
        (NativeText::OpenAiResponsesRequestFailed, NativeLang::ZhHant) => "OpenAI Responses 要求失敗",
        (NativeText::OpenAiResponsesRequestFailed, NativeLang::Fr) => "Échec de la requête OpenAI Responses",

        (NativeText::AnthropicRequestFailed, NativeLang::En) => "Anthropic request failed",
        (NativeText::AnthropicRequestFailed, NativeLang::PtBr) => "Falha na solicitação à Anthropic",
        (NativeText::AnthropicRequestFailed, NativeLang::ZhHans) => "Anthropic 请求失败",
        (NativeText::AnthropicRequestFailed, NativeLang::ZhHant) => "Anthropic 要求失敗",
        (NativeText::AnthropicRequestFailed, NativeLang::Fr) => "Échec de la requête Anthropic",

        (NativeText::GoogleSearchNeedsGemini3, NativeLang::En) => "Google Search with agent tools requires Gemini 3. Select a Gemini 3 model, remove the agent's tools, or disable Google Search in AI settings.",
        (NativeText::GoogleSearchNeedsGemini3, NativeLang::PtBr) => "A Pesquisa Google com ferramentas do agente requer o Gemini 3. Selecione um modelo Gemini 3, remova as ferramentas do agente ou desative a Pesquisa Google nas configurações de IA.",
        (NativeText::GoogleSearchNeedsGemini3, NativeLang::ZhHans) => "将 Google 搜索与智能体工具一起使用需要 Gemini 3。请选择 Gemini 3 模型、移除智能体的工具，或在 AI 设置中停用 Google 搜索。",
        (NativeText::GoogleSearchNeedsGemini3, NativeLang::ZhHant) => "將 Google 搜尋與代理工具一起使用需要 Gemini 3。請選擇 Gemini 3 模型、移除代理的工具，或在 AI 設定中停用 Google 搜尋。",
        (NativeText::GoogleSearchNeedsGemini3, NativeLang::Fr) => "La recherche Google avec les outils de l'agent nécessite Gemini 3. Choisissez un modèle Gemini 3, retirez les outils de l'agent ou désactivez la recherche Google dans les réglages IA.",

        (NativeText::CustomProviderBaseUrlEmpty, NativeLang::En) => "Custom provider base URL must not be empty",
        (NativeText::CustomProviderBaseUrlEmpty, NativeLang::PtBr) => "A URL base do provedor personalizado não pode estar vazia",
        (NativeText::CustomProviderBaseUrlEmpty, NativeLang::ZhHans) => "自定义服务商的基础 URL 不能为空",
        (NativeText::CustomProviderBaseUrlEmpty, NativeLang::ZhHant) => "自訂供應商的基礎 URL 不得為空",
        (NativeText::CustomProviderBaseUrlEmpty, NativeLang::Fr) => "L'URL de base du fournisseur personnalisé ne doit pas être vide",

        (NativeText::CustomProviderRequiresBaseUrl, NativeLang::En) => "Custom AI provider requires a base URL",
        (NativeText::CustomProviderRequiresBaseUrl, NativeLang::PtBr) => "O provedor de IA personalizado requer uma URL base",
        (NativeText::CustomProviderRequiresBaseUrl, NativeLang::ZhHans) => "自定义 AI 服务商需要基础 URL",
        (NativeText::CustomProviderRequiresBaseUrl, NativeLang::ZhHant) => "自訂 AI 供應商需要基礎 URL",
        (NativeText::CustomProviderRequiresBaseUrl, NativeLang::Fr) => "Le fournisseur IA personnalisé nécessite une URL de base",

        (NativeText::CredentialSecretEmpty, NativeLang::En) => "Credential secret cannot be empty",
        (NativeText::CredentialSecretEmpty, NativeLang::PtBr) => "O segredo da credencial não pode ficar vazio",
        (NativeText::CredentialSecretEmpty, NativeLang::ZhHans) => "凭据密钥不能为空",
        (NativeText::CredentialSecretEmpty, NativeLang::ZhHant) => "憑證密鑰不得為空",
        (NativeText::CredentialSecretEmpty, NativeLang::Fr) => "La clé secrète ne peut pas être vide",

        (NativeText::ProviderNotConfigured, NativeLang::En) => "provider '{provider}' is not configured",
        (NativeText::ProviderNotConfigured, NativeLang::PtBr) => "o provedor '{provider}' não está configurado",
        (NativeText::ProviderNotConfigured, NativeLang::ZhHans) => "服务商“{provider}”尚未配置",
        (NativeText::ProviderNotConfigured, NativeLang::ZhHant) => "供應商「{provider}」尚未設定",
        (NativeText::ProviderNotConfigured, NativeLang::Fr) => "le fournisseur « {provider} » n'est pas configuré",

        (NativeText::ProviderDisabled, NativeLang::En) => "provider '{provider}' is disabled",
        (NativeText::ProviderDisabled, NativeLang::PtBr) => "o provedor '{provider}' está desativado",
        (NativeText::ProviderDisabled, NativeLang::ZhHans) => "服务商“{provider}”已停用",
        (NativeText::ProviderDisabled, NativeLang::ZhHant) => "供應商「{provider}」已停用",
        (NativeText::ProviderDisabled, NativeLang::Fr) => "le fournisseur « {provider} » est désactivé",

        (NativeText::ProviderNotRegistered, NativeLang::En) => "provider '{provider}' is not registered",
        (NativeText::ProviderNotRegistered, NativeLang::PtBr) => "o provedor '{provider}' não está registrado",
        (NativeText::ProviderNotRegistered, NativeLang::ZhHans) => "服务商“{provider}”未注册",
        (NativeText::ProviderNotRegistered, NativeLang::ZhHant) => "供應商「{provider}」未註冊",
        (NativeText::ProviderNotRegistered, NativeLang::Fr) => "le fournisseur « {provider} » n'est pas enregistré",

        (NativeText::ProviderCliModeUnsupported, NativeLang::En) => "CLI mode is not supported for provider '{provider}'",
        (NativeText::ProviderCliModeUnsupported, NativeLang::PtBr) => "O modo CLI não é compatível com o provedor '{provider}'",
        (NativeText::ProviderCliModeUnsupported, NativeLang::ZhHans) => "服务商“{provider}”不支持 CLI 模式",
        (NativeText::ProviderCliModeUnsupported, NativeLang::ZhHant) => "供應商「{provider}」不支援 CLI 模式",
        (NativeText::ProviderCliModeUnsupported, NativeLang::Fr) => "Le mode CLI n'est pas pris en charge pour le fournisseur « {provider} »",

        (NativeText::ProviderCliNotFound, NativeLang::En) => "CLI executable for provider '{provider}' was not found",
        (NativeText::ProviderCliNotFound, NativeLang::PtBr) => "O executável da CLI do provedor '{provider}' não foi encontrado",
        (NativeText::ProviderCliNotFound, NativeLang::ZhHans) => "未找到服务商“{provider}”的 CLI 可执行文件",
        (NativeText::ProviderCliNotFound, NativeLang::ZhHant) => "找不到供應商「{provider}」的 CLI 執行檔",
        (NativeText::ProviderCliNotFound, NativeLang::Fr) => "Exécutable CLI introuvable pour le fournisseur « {provider} »",

        (NativeText::ProviderApiKeyUndecryptable, NativeLang::En) => "API key for provider '{provider}' cannot be decrypted (keychain master key changed). Please re-enter your key in Settings → AI.",
        (NativeText::ProviderApiKeyUndecryptable, NativeLang::PtBr) => "Não foi possível descriptografar a chave de API do provedor '{provider}' (a chave mestra do chaveiro mudou). Insira sua chave novamente em Configurações → IA.",
        (NativeText::ProviderApiKeyUndecryptable, NativeLang::ZhHans) => "无法解密服务商“{provider}”的 API 密钥（钥匙串主密钥已更改）。请在“设置 → AI”中重新输入密钥。",
        (NativeText::ProviderApiKeyUndecryptable, NativeLang::ZhHant) => "無法解密供應商「{provider}」的 API 金鑰（鑰匙圈主金鑰已變更）。請在「設定 → AI」中重新輸入金鑰。",
        (NativeText::ProviderApiKeyUndecryptable, NativeLang::Fr) => "Impossible de déchiffrer la clé API du fournisseur « {provider} » (la clé maîtresse du trousseau a changé). Saisissez à nouveau votre clé dans Réglages → IA.",

        (NativeText::ProviderApiKeyUndecryptableShort, NativeLang::En) => "API key for provider '{provider}' cannot be decrypted.",
        (NativeText::ProviderApiKeyUndecryptableShort, NativeLang::PtBr) => "Não foi possível descriptografar a chave de API do provedor '{provider}'.",
        (NativeText::ProviderApiKeyUndecryptableShort, NativeLang::ZhHans) => "无法解密服务商“{provider}”的 API 密钥。",
        (NativeText::ProviderApiKeyUndecryptableShort, NativeLang::ZhHant) => "無法解密供應商「{provider}」的 API 金鑰。",
        (NativeText::ProviderApiKeyUndecryptableShort, NativeLang::Fr) => "Impossible de déchiffrer la clé API du fournisseur « {provider} ».",

        (NativeText::ProviderApiKeyMissing, NativeLang::En) => "API key for provider '{provider}' is not configured",
        (NativeText::ProviderApiKeyMissing, NativeLang::PtBr) => "A chave de API do provedor '{provider}' não está configurada",
        (NativeText::ProviderApiKeyMissing, NativeLang::ZhHans) => "尚未配置服务商“{provider}”的 API 密钥",
        (NativeText::ProviderApiKeyMissing, NativeLang::ZhHant) => "尚未設定供應商「{provider}」的 API 金鑰",
        (NativeText::ProviderApiKeyMissing, NativeLang::Fr) => "Aucune clé API n'est configurée pour le fournisseur « {provider} »",

        (NativeText::ProviderBaseUrlMissing, NativeLang::En) => "Base URL for provider '{provider}' is not configured",
        (NativeText::ProviderBaseUrlMissing, NativeLang::PtBr) => "A URL base do provedor '{provider}' não está configurada",
        (NativeText::ProviderBaseUrlMissing, NativeLang::ZhHans) => "尚未配置服务商“{provider}”的基础 URL",
        (NativeText::ProviderBaseUrlMissing, NativeLang::ZhHant) => "尚未設定供應商「{provider}」的基礎 URL",
        (NativeText::ProviderBaseUrlMissing, NativeLang::Fr) => "Aucune URL de base n'est configurée pour le fournisseur « {provider} »",

        (NativeText::ExtensionAiProviderFailed, NativeLang::En) => "Extension AI provider failed: {error}",
        (NativeText::ExtensionAiProviderFailed, NativeLang::PtBr) => "Falha no provedor de IA da extensão: {error}",
        (NativeText::ExtensionAiProviderFailed, NativeLang::ZhHans) => "扩展 AI 服务商出错：{error}",
        (NativeText::ExtensionAiProviderFailed, NativeLang::ZhHant) => "擴充套件 AI 供應商發生錯誤：{error}",
        (NativeText::ExtensionAiProviderFailed, NativeLang::Fr) => "Échec du fournisseur IA de l'extension : {error}",

        (NativeText::ScriptDirUnreadable, NativeLang::En) => "Could not read watched directory: {error}",
        (NativeText::ScriptDirUnreadable, NativeLang::PtBr) => "Não foi possível ler a pasta monitorada: {error}",
        (NativeText::ScriptDirUnreadable, NativeLang::ZhHans) => "无法读取监视的目录：{error}",
        (NativeText::ScriptDirUnreadable, NativeLang::ZhHant) => "無法讀取監看的目錄：{error}",
        (NativeText::ScriptDirUnreadable, NativeLang::Fr) => "Impossible de lire le dossier surveillé : {error}",

        (NativeText::ScriptMetadataUnreadable, NativeLang::En) => "Could not inspect file metadata: {error}",
        (NativeText::ScriptMetadataUnreadable, NativeLang::PtBr) => "Não foi possível ler os metadados do arquivo: {error}",
        (NativeText::ScriptMetadataUnreadable, NativeLang::ZhHans) => "无法读取文件元数据：{error}",
        (NativeText::ScriptMetadataUnreadable, NativeLang::ZhHant) => "無法讀取檔案中繼資料：{error}",
        (NativeText::ScriptMetadataUnreadable, NativeLang::Fr) => "Impossible de lire les métadonnées du fichier : {error}",

        (NativeText::ScriptPathUnavailable, NativeLang::En) => "Could not resolve file path: {error}",
        (NativeText::ScriptPathUnavailable, NativeLang::PtBr) => "Não foi possível resolver o caminho do arquivo: {error}",
        (NativeText::ScriptPathUnavailable, NativeLang::ZhHans) => "无法解析文件路径：{error}",
        (NativeText::ScriptPathUnavailable, NativeLang::ZhHant) => "無法解析檔案路徑：{error}",
        (NativeText::ScriptPathUnavailable, NativeLang::Fr) => "Impossible de résoudre le chemin du fichier : {error}",

        (NativeText::ScriptNotExecutable, NativeLang::En) => "Script is not executable",
        (NativeText::ScriptNotExecutable, NativeLang::PtBr) => "O script não é executável",
        (NativeText::ScriptNotExecutable, NativeLang::ZhHans) => "脚本不可执行",
        (NativeText::ScriptNotExecutable, NativeLang::ZhHant) => "指令碼無法執行",
        (NativeText::ScriptNotExecutable, NativeLang::Fr) => "Le script n'est pas exécutable",

        (NativeText::ScriptContentUnreadable, NativeLang::En) => "Could not read script: {error}",
        (NativeText::ScriptContentUnreadable, NativeLang::PtBr) => "Não foi possível ler o script: {error}",
        (NativeText::ScriptContentUnreadable, NativeLang::ZhHans) => "无法读取脚本：{error}",
        (NativeText::ScriptContentUnreadable, NativeLang::ZhHant) => "無法讀取指令碼：{error}",
        (NativeText::ScriptContentUnreadable, NativeLang::Fr) => "Impossible de lire le script : {error}",

        (NativeText::ScriptInvalidHeader, NativeLang::En) => "Invalid script header: {error}",
        (NativeText::ScriptInvalidHeader, NativeLang::PtBr) => "Cabeçalho do script inválido: {error}",
        (NativeText::ScriptInvalidHeader, NativeLang::ZhHans) => "脚本头无效：{error}",
        (NativeText::ScriptInvalidHeader, NativeLang::ZhHant) => "指令碼標頭無效：{error}",
        (NativeText::ScriptInvalidHeader, NativeLang::Fr) => "En-tête de script non valide : {error}",

        (NativeText::ScriptHeaderArgumentJson, NativeLang::En) => "invalid argument JSON on line {line}: {message}",
        (NativeText::ScriptHeaderArgumentJson, NativeLang::PtBr) => "JSON de argumento inválido na linha {line}: {message}",
        (NativeText::ScriptHeaderArgumentJson, NativeLang::ZhHans) => "第 {line} 行的参数 JSON 无效：{message}",
        (NativeText::ScriptHeaderArgumentJson, NativeLang::ZhHant) => "第 {line} 行的參數 JSON 無效：{message}",
        (NativeText::ScriptHeaderArgumentJson, NativeLang::Fr) => "JSON d'argument non valide à la ligne {line} : {message}",

        (NativeText::ScriptHeaderArgumentIndex, NativeLang::En) => "argument index {index} out of range (must be 1, 2, or 3)",
        (NativeText::ScriptHeaderArgumentIndex, NativeLang::PtBr) => "índice de argumento {index} fora do intervalo (deve ser 1, 2 ou 3)",
        (NativeText::ScriptHeaderArgumentIndex, NativeLang::ZhHans) => "参数索引 {index} 超出范围（必须为 1、2 或 3）",
        (NativeText::ScriptHeaderArgumentIndex, NativeLang::ZhHant) => "參數索引 {index} 超出範圍（必須為 1、2 或 3）",
        (NativeText::ScriptHeaderArgumentIndex, NativeLang::Fr) => "index d'argument {index} hors limites (1, 2 ou 3 attendu)",

        (NativeText::ScriptHeaderDuplicateIndex, NativeLang::En) => "duplicate argument index {index}",
        (NativeText::ScriptHeaderDuplicateIndex, NativeLang::PtBr) => "índice de argumento {index} duplicado",
        (NativeText::ScriptHeaderDuplicateIndex, NativeLang::ZhHans) => "参数索引 {index} 重复",
        (NativeText::ScriptHeaderDuplicateIndex, NativeLang::ZhHant) => "參數索引 {index} 重複",
        (NativeText::ScriptHeaderDuplicateIndex, NativeLang::Fr) => "index d'argument {index} en double",

        (NativeText::ScriptHeaderInvalidMode, NativeLang::En) => "invalid mode value '{value}' (expected silent | compact | fullOutput | inline)",
        (NativeText::ScriptHeaderInvalidMode, NativeLang::PtBr) => "valor de modo '{value}' inválido (esperado: silent | compact | fullOutput | inline)",
        (NativeText::ScriptHeaderInvalidMode, NativeLang::ZhHans) => "模式值“{value}”无效（应为 silent | compact | fullOutput | inline）",
        (NativeText::ScriptHeaderInvalidMode, NativeLang::ZhHant) => "模式值「{value}」無效（應為 silent | compact | fullOutput | inline）",
        (NativeText::ScriptHeaderInvalidMode, NativeLang::Fr) => "valeur de mode '{value}' non valide (attendu : silent | compact | fullOutput | inline)",

        (NativeText::ScriptHeaderRequireAnyOf, NativeLang::En) => "invalid requireAnyOf on line {line}: {message}",
        (NativeText::ScriptHeaderRequireAnyOf, NativeLang::PtBr) => "requireAnyOf inválido na linha {line}: {message}",
        (NativeText::ScriptHeaderRequireAnyOf, NativeLang::ZhHans) => "第 {line} 行的 requireAnyOf 无效：{message}",
        (NativeText::ScriptHeaderRequireAnyOf, NativeLang::ZhHant) => "第 {line} 行的 requireAnyOf 無效：{message}",
        (NativeText::ScriptHeaderRequireAnyOf, NativeLang::Fr) => "requireAnyOf non valide à la ligne {line} : {message}",

        (NativeText::ScriptHeaderRefreshTime, NativeLang::En) => "invalid refreshTime '{value}' (expected <N><s|m|h|d>, e.g. 30s or 5m)",
        (NativeText::ScriptHeaderRefreshTime, NativeLang::PtBr) => "refreshTime '{value}' inválido (esperado <N><s|m|h|d>, ex.: 30s ou 5m)",
        (NativeText::ScriptHeaderRefreshTime, NativeLang::ZhHans) => "refreshTime“{value}”无效（应为 <N><s|m|h|d>，例如 30s 或 5m）",
        (NativeText::ScriptHeaderRefreshTime, NativeLang::ZhHant) => "refreshTime「{value}」無效（應為 <N><s|m|h|d>，例如 30s 或 5m）",
        (NativeText::ScriptHeaderRefreshTime, NativeLang::Fr) => "refreshTime '{value}' non valide (format attendu <N><s|m|h|d>, par ex. 30s ou 5m)",

        (NativeText::McpTransportError, NativeLang::En) => "transport error: {error}",
        (NativeText::McpTransportError, NativeLang::PtBr) => "erro de transporte: {error}",
        (NativeText::McpTransportError, NativeLang::ZhHans) => "传输错误：{error}",
        (NativeText::McpTransportError, NativeLang::ZhHant) => "傳輸錯誤：{error}",
        (NativeText::McpTransportError, NativeLang::Fr) => "erreur de transport : {error}",

        (NativeText::McpProtocolError, NativeLang::En) => "protocol error: {error}",
        (NativeText::McpProtocolError, NativeLang::PtBr) => "erro de protocolo: {error}",
        (NativeText::McpProtocolError, NativeLang::ZhHans) => "协议错误：{error}",
        (NativeText::McpProtocolError, NativeLang::ZhHant) => "通訊協定錯誤：{error}",
        (NativeText::McpProtocolError, NativeLang::Fr) => "erreur de protocole : {error}",

        (NativeText::McpRpcError, NativeLang::En) => "rpc error {code}: {message}",
        (NativeText::McpRpcError, NativeLang::PtBr) => "erro RPC {code}: {message}",
        (NativeText::McpRpcError, NativeLang::ZhHans) => "RPC 错误 {code}：{message}",
        (NativeText::McpRpcError, NativeLang::ZhHant) => "RPC 錯誤 {code}：{message}",
        (NativeText::McpRpcError, NativeLang::Fr) => "erreur RPC {code} : {message}",

        (NativeText::McpEarlyExit, NativeLang::En) => "server exited before completing handshake",
        (NativeText::McpEarlyExit, NativeLang::PtBr) => "o servidor foi encerrado antes de concluir o handshake",
        (NativeText::McpEarlyExit, NativeLang::ZhHans) => "服务器在完成握手前已退出",
        (NativeText::McpEarlyExit, NativeLang::ZhHant) => "伺服器在完成交握前已結束",
        (NativeText::McpEarlyExit, NativeLang::Fr) => "le serveur s'est arrêté avant la fin de la négociation",

        (NativeText::McpTimeout, NativeLang::En) => "timeout waiting for response to id={id}",
        (NativeText::McpTimeout, NativeLang::PtBr) => "tempo esgotado aguardando a resposta para id={id}",
        (NativeText::McpTimeout, NativeLang::ZhHans) => "等待 id={id} 的响应超时",
        (NativeText::McpTimeout, NativeLang::ZhHant) => "等待 id={id} 的回應逾時",
        (NativeText::McpTimeout, NativeLang::Fr) => "délai dépassé en attendant la réponse à id={id}",

        (NativeText::ExtPlatformUnsupported, NativeLang::En) => "Extension '{name}' does not support {platform} (supported: {supported})",
        (NativeText::ExtPlatformUnsupported, NativeLang::PtBr) => "A extensão '{name}' não é compatível com {platform} (compatíveis: {supported})",
        (NativeText::ExtPlatformUnsupported, NativeLang::ZhHans) => "扩展“{name}”不支持 {platform}（支持：{supported}）",
        (NativeText::ExtPlatformUnsupported, NativeLang::ZhHant) => "擴充套件「{name}」不支援 {platform}（支援：{supported}）",
        (NativeText::ExtPlatformUnsupported, NativeLang::Fr) => "L'extension « {name} » ne prend pas en charge {platform} (compatibles : {supported})",

        (NativeText::ExtPlatformUnsupportedDeclared, NativeLang::En) => "Extension '{name}' does not support {platform} (supported platforms: {supported})",
        (NativeText::ExtPlatformUnsupportedDeclared, NativeLang::PtBr) => "A extensão '{name}' não é compatível com {platform} (plataformas compatíveis: {supported})",
        (NativeText::ExtPlatformUnsupportedDeclared, NativeLang::ZhHans) => "扩展“{name}”不支持 {platform}（支持的平台：{supported}）",
        (NativeText::ExtPlatformUnsupportedDeclared, NativeLang::ZhHant) => "擴充套件「{name}」不支援 {platform}（支援的平台：{supported}）",
        (NativeText::ExtPlatformUnsupportedDeclared, NativeLang::Fr) => "L'extension « {name} » ne prend pas en charge {platform} (plateformes compatibles : {supported})",

        (NativeText::ExtSupportedNone, NativeLang::En) => "none",
        (NativeText::ExtSupportedNone, NativeLang::PtBr) => "nenhuma",
        (NativeText::ExtSupportedNone, NativeLang::ZhHans) => "无",
        (NativeText::ExtSupportedNone, NativeLang::ZhHant) => "無",
        (NativeText::ExtSupportedNone, NativeLang::Fr) => "aucune",

        (NativeText::ExtSupportedNoneDeclared, NativeLang::En) => "none declared",
        (NativeText::ExtSupportedNoneDeclared, NativeLang::PtBr) => "nenhuma declarada",
        (NativeText::ExtSupportedNoneDeclared, NativeLang::ZhHans) => "未声明",
        (NativeText::ExtSupportedNoneDeclared, NativeLang::ZhHant) => "未宣告",
        (NativeText::ExtSupportedNoneDeclared, NativeLang::Fr) => "aucune déclarée",

        (NativeText::ExtAlreadyInstalled, NativeLang::En) => "Extension '{id}' v{version} is already installed (same or newer version)",
        (NativeText::ExtAlreadyInstalled, NativeLang::PtBr) => "A extensão '{id}' v{version} já está instalada (versão igual ou mais recente)",
        (NativeText::ExtAlreadyInstalled, NativeLang::ZhHans) => "扩展“{id}” v{version} 已安装（相同或更新的版本）",
        (NativeText::ExtAlreadyInstalled, NativeLang::ZhHant) => "擴充套件「{id}」v{version} 已安裝（相同或更新的版本）",
        (NativeText::ExtAlreadyInstalled, NativeLang::Fr) => "L'extension « {id} » v{version} est déjà installée (version identique ou plus récente)",

        (NativeText::ExtFileMustBeAsyar, NativeLang::En) => "File must have .asyar extension",
        (NativeText::ExtFileMustBeAsyar, NativeLang::PtBr) => "O arquivo deve ter a extensão .asyar",
        (NativeText::ExtFileMustBeAsyar, NativeLang::ZhHans) => "文件扩展名必须为 .asyar",
        (NativeText::ExtFileMustBeAsyar, NativeLang::ZhHant) => "檔案副檔名必須為 .asyar",
        (NativeText::ExtFileMustBeAsyar, NativeLang::Fr) => "Le fichier doit avoir l'extension .asyar",

        (NativeText::ExtChecksumMismatch, NativeLang::En) => "Checksum mismatch! Expected: {expected}, Calculated: {calculated}",
        (NativeText::ExtChecksumMismatch, NativeLang::PtBr) => "Checksum não confere! Esperado: {expected}, calculado: {calculated}",
        (NativeText::ExtChecksumMismatch, NativeLang::ZhHans) => "校验和不匹配！预期：{expected}，计算结果：{calculated}",
        (NativeText::ExtChecksumMismatch, NativeLang::ZhHant) => "總和檢查碼不符！預期：{expected}，計算結果：{calculated}",
        (NativeText::ExtChecksumMismatch, NativeLang::Fr) => "Somme de contrôle incorrecte ! Attendue : {expected}, calculée : {calculated}",

        (NativeText::ExtHttpsRequired, NativeLang::En) => "Extension downloads require HTTPS. Insecure URL: {url}",
        (NativeText::ExtHttpsRequired, NativeLang::PtBr) => "Downloads de extensões exigem HTTPS. URL insegura: {url}",
        (NativeText::ExtHttpsRequired, NativeLang::ZhHans) => "下载扩展需要使用 HTTPS。不安全的 URL：{url}",
        (NativeText::ExtHttpsRequired, NativeLang::ZhHant) => "下載擴充套件需要使用 HTTPS。不安全的 URL：{url}",
        (NativeText::ExtHttpsRequired, NativeLang::Fr) => "Les téléchargements d'extensions nécessitent HTTPS. URL non sécurisée : {url}",
    }
}

/// Like [`native_text`], then fills `{name}` placeholders from `args` in a
/// single pass (substituted values are never re-scanned). Unknown
/// placeholders are left as written.
pub fn native_format(locale: &ParsedLocale, key: NativeText, args: &[(&str, &str)]) -> String {
    fill_placeholders(native_text(locale, key), args)
}

fn fill_placeholders(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match after.find('}') {
            Some(close) => {
                let name = &after[..close];
                match args.iter().find(|(k, _)| *k == name) {
                    Some((_, value)) => out.push_str(value),
                    None => {
                        out.push('{');
                        out.push_str(name);
                        out.push('}');
                    }
                }
                rest = &after[close + 1..];
            }
            None => {
                out.push_str(&rest[open..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// The locale native strings should follow: the managed `LocaleService` when
/// registered, otherwise the host locale (early boot, tests).
pub fn app_locale<R: tauri::Runtime, M: tauri::Manager<R>>(manager: &M) -> ParsedLocale {
    manager
        .try_state::<super::LocaleService>()
        .map(|svc| svc.current_locale())
        .unwrap_or_else(super::detect)
}

/// Looks up `key` using the app's managed `LocaleService`, falling back to the
/// host locale when the service is not registered yet (early boot, tests).
pub fn app_native_text<R: tauri::Runtime, M: tauri::Manager<R>>(
    manager: &M,
    key: NativeText,
) -> &'static str {
    native_text(&app_locale(manager), key)
}

/// The locale for messages built deep in the core, where no app handle is
/// reachable (errors raised inside AI clients, scanners, installers). It is the
/// host locale, which is also what the frontend follows. Unit tests pin English
/// so assertions on error text don't depend on the developer's machine.
pub fn host_locale() -> ParsedLocale {
    #[cfg(test)]
    {
        ParsedLocale::parse("en-US").expect("valid locale tag")
    }
    #[cfg(not(test))]
    {
        super::detect()
    }
}

/// [`native_text`] in the [`host_locale`].
pub fn host_text(key: NativeText) -> &'static str {
    native_text(&host_locale(), key)
}

/// [`native_format`] in the [`host_locale`].
pub fn host_format(key: NativeText, args: &[(&str, &str)]) -> String {
    native_format(&host_locale(), key, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lang(tag: &str) -> NativeLang {
        NativeLang::from_locale(&ParsedLocale::parse(tag).unwrap())
    }

    #[test]
    fn picks_language_tables() {
        assert_eq!(lang("en-US"), NativeLang::En);
        assert_eq!(lang("de-DE"), NativeLang::En);
        assert_eq!(lang("pt-BR"), NativeLang::PtBr);
        assert_eq!(lang("pt-PT"), NativeLang::PtBr);
        assert_eq!(lang("fr-FR"), NativeLang::Fr);
        assert_eq!(lang("fr-CA"), NativeLang::Fr);
        assert_eq!(lang("fr"), NativeLang::Fr);
    }

    #[test]
    fn chinese_is_split_by_script_then_region() {
        assert_eq!(lang("zh-CN"), NativeLang::ZhHans);
        assert_eq!(lang("zh"), NativeLang::ZhHans);
        assert_eq!(lang("zh-Hans-CN"), NativeLang::ZhHans);
        assert_eq!(lang("zh-Hant"), NativeLang::ZhHant);
        assert_eq!(lang("zh-Hant-TW"), NativeLang::ZhHant);
        assert_eq!(lang("zh-TW"), NativeLang::ZhHant);
        assert_eq!(lang("zh-HK"), NativeLang::ZhHant);
        // An explicit Simplified script beats a Traditional-leaning region.
        assert_eq!(lang("zh-Hans-TW"), NativeLang::ZhHans);
    }

    #[test]
    fn every_key_is_translated_and_non_empty_in_every_language() {
        for tag in ["en-US", "pt-BR", "zh-CN", "zh-Hant", "fr-FR"] {
            let locale = ParsedLocale::parse(tag).unwrap();
            for &key in NativeText::ALL {
                assert!(!native_text(&locale, key).is_empty(), "{tag} {key:?}");
            }
        }
    }

    #[test]
    fn english_matches_the_previous_hardcoded_strings() {
        let en = ParsedLocale::parse("en-US").unwrap();
        assert_eq!(native_text(&en, NativeText::TrayQuit), "Quit Asyar");
        assert_eq!(
            native_text(&en, NativeText::TrayCheckUpdates),
            "Check for Updates"
        );
        assert_eq!(native_text(&en, NativeText::TraySettings), "Settings");
        assert_eq!(native_text(&en, NativeText::StickyNoteTitle), "Sticky Note");
        assert_eq!(
            native_text(&en, NativeText::OnboardingTitle),
            "Welcome to Asyar"
        );
        assert_eq!(
            native_text(&en, NativeText::OcrNoText),
            "No text found in selected area"
        );
        assert_eq!(
            native_text(&en, NativeText::OcrCopied),
            "Text copied to clipboard"
        );
        assert_eq!(native_text(&en, NativeText::RunFailedTitle), "Run failed");
        assert_eq!(
            native_text(&en, NativeText::ScriptFinishedTitle),
            "Script finished"
        );
        assert_eq!(native_text(&en, NativeText::RunActionOpen), "Open");
        assert_eq!(
            native_text(&en, NativeText::RunActionViewOutput),
            "View output"
        );
        assert_eq!(native_text(&en, NativeText::RunNoOutput), "(no output)");
        assert_eq!(native_text(&en, NativeText::NotificationClose), "Close");
        assert_eq!(
            native_text(&en, NativeText::DialogFilterProfile),
            "Asyar Profile"
        );
        assert_eq!(
            native_text(&en, NativeText::DialogFilterPackage),
            "Asyar Package"
        );
        assert_eq!(
            native_text(&en, NativeText::ExtBuilderApiKeyMissing),
            "Anthropic API key is not configured. Please set your API key in Settings → AI."
        );
        assert_eq!(
            native_text(&en, NativeText::ExtBuilderApiKeyUndecryptable),
            "API key for Anthropic cannot be decrypted (keychain master key changed). Please re-enter your key in Settings → AI."
        );
    }

    #[test]
    fn english_templates_match_the_previous_format_strings() {
        let en = ParsedLocale::parse("en-US").unwrap();
        assert_eq!(
            native_format(
                &en,
                NativeText::OcrCopiedPreview,
                &[("text", "Hello World")]
            ),
            "Text copied: \"Hello World\""
        );
        assert_eq!(
            native_format(
                &en,
                NativeText::CliBinaryNotFound,
                &[("name", "Claude Code (claude)")]
            ),
            "Claude Code (claude) CLI binary not found on this machine."
        );
        assert_eq!(
            native_format(
                &en,
                NativeText::ProviderRemovalBlocked,
                &[("provider_name", "Anthropic"), ("agent_names", "A, B")]
            ),
            "Can't remove Anthropic — it's the last configured provider and these agents still \
             use it: A, B. Reassign or delete them first."
        );
    }

    #[test]
    fn templates_keep_the_same_placeholders_in_every_language() {
        let expected: &[(NativeText, &[&str])] = &[
            (NativeText::OcrCopiedPreview, &["{text}"]),
            (NativeText::CliBinaryNotFound, &["{name}"]),
            (
                NativeText::ProviderRemovalBlocked,
                &["{provider_name}", "{agent_names}"],
            ),
        ];
        for tag in ["en-US", "pt-BR", "zh-CN", "zh-Hant", "fr-FR"] {
            let locale = ParsedLocale::parse(tag).unwrap();
            for (key, placeholders) in expected {
                let text = native_text(&locale, *key);
                for ph in *placeholders {
                    assert!(text.contains(ph), "{tag} {key:?} lacks {ph}");
                }
            }
        }
    }

    fn placeholders(text: &str) -> Vec<&str> {
        let mut found: Vec<&str> = Vec::new();
        let mut rest = text;
        while let Some(open) = rest.find('{') {
            let after = &rest[open + 1..];
            let Some(close) = after.find('}') else { break };
            found.push(&after[..close]);
            rest = &after[close + 1..];
        }
        found.sort_unstable();
        found
    }

    #[test]
    fn every_key_has_the_english_placeholders_in_every_language() {
        let en = ParsedLocale::parse("en-US").unwrap();
        for tag in ["pt-BR", "zh-CN", "zh-Hant", "fr-FR"] {
            let locale = ParsedLocale::parse(tag).unwrap();
            for &key in NativeText::ALL {
                assert_eq!(
                    placeholders(native_text(&locale, key)),
                    placeholders(native_text(&en, key)),
                    "{tag} {key:?}"
                );
            }
        }
    }

    #[test]
    fn english_error_messages_match_the_previous_strings() {
        let en = ParsedLocale::parse("en-US").unwrap();
        assert_eq!(
            native_text(&en, NativeText::CodexLoginNotFound),
            "Codex login not found. Please log in using the Codex CLI first."
        );
        assert_eq!(
            native_text(&en, NativeText::ClaudeCodeReportedError),
            "Claude Code reported an error"
        );
        assert_eq!(
            native_format(&en, NativeText::CliExecFailed, &[("error", "denied")]),
            "Failed to execute CLI binary: denied"
        );
        assert_eq!(
            native_format(
                &en,
                NativeText::ProviderApiKeyMissing,
                &[("provider", "openai")]
            ),
            "API key for provider 'openai' is not configured"
        );
        assert_eq!(
            native_format(&en, NativeText::ScriptInvalidHeader, &[("error", "x")]),
            "Invalid script header: x"
        );
        assert_eq!(
            native_format(&en, NativeText::McpTimeout, &[("id", "3")]),
            "timeout waiting for response to id=3"
        );
    }

    #[test]
    fn host_text_is_english_under_test() {
        assert_eq!(
            host_text(NativeText::ScriptNotExecutable),
            "Script is not executable"
        );
        assert_eq!(
            host_format(NativeText::CodexStreamReadError, &[("error", "eof")]),
            "Stream read error: eof"
        );
    }

    #[test]
    fn fill_placeholders_is_single_pass_and_keeps_unknown_names() {
        assert_eq!(
            fill_placeholders("{a} and {b}", &[("a", "{b}"), ("b", "x")]),
            "{b} and x"
        );
        assert_eq!(fill_placeholders("keep {unknown}", &[]), "keep {unknown}");
        assert_eq!(fill_placeholders("open { brace", &[]), "open { brace");
        assert_eq!(fill_placeholders("no braces", &[("a", "b")]), "no braces");
    }

    #[test]
    fn localised_samples() {
        let fr = ParsedLocale::parse("fr-FR").unwrap();
        assert_eq!(
            native_text(&fr, NativeText::RunFailedTitle),
            "Échec de l'exécution"
        );
        let zh = ParsedLocale::parse("zh-TW").unwrap();
        assert_eq!(native_text(&zh, NativeText::NotificationClose), "關閉");
        assert_eq!(
            native_format(&fr, NativeText::OcrCopiedPreview, &[("text", "abc")]),
            "Texte copié : « abc »"
        );
        assert_eq!(
            native_text(&fr, NativeText::CodexLoginNotFound),
            "Connexion Codex introuvable. Connectez-vous d'abord avec la CLI Codex."
        );
    }
}
