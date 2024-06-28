#[derive(Debug, PartialEq)]
pub enum PinentryOption {
    FormattedPassphrase,
    FormattedPassphraseHint(String),
    TtyType(String),
    TtYName(String),
    LccType(String),
    DefaultOk(String),
    DefaultCancel(String),
    DefaultPrompt(String),
    AllowExternalPasswordCache,
    UnknownOption,
}

impl PinentryOption {
    pub fn parse(input: &str) -> PinentryOption {
        let parts: Vec<&str> = input.split("=").collect();
        let get_second_part = |default: &str| parts.get(1).unwrap_or(&default).to_string();
        match parts[0] {
            "formatted-passphrase" => PinentryOption::FormattedPassphrase,
            "formatted-passphrase-hint" => {
                PinentryOption::FormattedPassphraseHint(get_second_part(""))
            }
            "ttytype" => PinentryOption::TtyType(get_second_part("")),
            "ttyname" => PinentryOption::TtYName(get_second_part("")),
            "lc-ctype" => PinentryOption::LccType(get_second_part("")),
            "default-ok" => PinentryOption::DefaultOk(get_second_part("")),
            "default-cancel" => PinentryOption::DefaultCancel(get_second_part("")),
            "default-prompt" => PinentryOption::DefaultPrompt(get_second_part("")),
            "allow-external-password-cache" => PinentryOption::AllowExternalPasswordCache,
            _ => PinentryOption::UnknownOption,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_formatted_passphrase_option() {
        assert_eq!(
            PinentryOption::parse("formatted-passphrase"),
            PinentryOption::FormattedPassphrase
        );
    }

    #[test]
    fn parses_formatted_passphrase_hint_option() {
        assert_eq!(
            PinentryOption::parse("formatted-passphrase-hint=test"),
            PinentryOption::FormattedPassphraseHint("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("formatted-passphrase-hint="),
            PinentryOption::FormattedPassphraseHint("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("formatted-passphrase-hint"),
            PinentryOption::FormattedPassphraseHint("".to_string())
        );
    }

    #[test]
    fn parses_ttytype_option() {
        assert_eq!(
            PinentryOption::parse("ttytype=test"),
            PinentryOption::TtyType("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("ttytype="),
            PinentryOption::TtyType("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("ttytype"),
            PinentryOption::TtyType("".to_string())
        );
    }

    #[test]
    fn parses_ttyname_option() {
        assert_eq!(
            PinentryOption::parse("ttyname=test"),
            PinentryOption::TtYName("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("ttyname="),
            PinentryOption::TtYName("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("ttyname"),
            PinentryOption::TtYName("".to_string())
        );
    }

    #[test]
    fn parses_lc_ctype_option() {
        assert_eq!(
            PinentryOption::parse("lc-ctype=test"),
            PinentryOption::LccType("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("lc-ctype="),
            PinentryOption::LccType("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("lc-ctype"),
            PinentryOption::LccType("".to_string())
        );
    }

    #[test]
    fn parses_default_ok_option() {
        assert_eq!(
            PinentryOption::parse("default-ok=test"),
            PinentryOption::DefaultOk("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("default-ok="),
            PinentryOption::DefaultOk("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("default-ok"),
            PinentryOption::DefaultOk("".to_string())
        );
    }

    #[test]
    fn parses_default_cancel_option() {
        assert_eq!(
            PinentryOption::parse("default-cancel=test"),
            PinentryOption::DefaultCancel("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("default-cancel="),
            PinentryOption::DefaultCancel("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("default-cancel"),
            PinentryOption::DefaultCancel("".to_string())
        );
    }

    #[test]
    fn parses_default_prompt_option() {
        assert_eq!(
            PinentryOption::parse("default-prompt=test"),
            PinentryOption::DefaultPrompt("test".to_string())
        );
        assert_eq!(
            PinentryOption::parse("default-prompt="),
            PinentryOption::DefaultPrompt("".to_string())
        );
        assert_eq!(
            PinentryOption::parse("default-prompt"),
            PinentryOption::DefaultPrompt("".to_string())
        );
    }

    #[test]
    fn parses_allow_external_password_cache_option() {
        assert_eq!(
            PinentryOption::parse("allow-external-password-cache"),
            PinentryOption::AllowExternalPasswordCache
        );
    }

    #[test]
    fn returns_unknown_option_for_anything_else() {
        assert_eq!(PinentryOption::parse("foo"), PinentryOption::UnknownOption);
        assert_eq!(PinentryOption::parse(""), PinentryOption::UnknownOption);
    }
}
