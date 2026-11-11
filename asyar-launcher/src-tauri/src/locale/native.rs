//! Translations for the strings Rust renders natively (tray menu, window
//! titles, OS notifications, the HUD, file-dialog filters) or hands to the
//! frontend fully formed (a few command results). Everything else is localised in the frontend catalogs under
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
    }
}
