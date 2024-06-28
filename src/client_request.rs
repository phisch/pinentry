use crate::option::PinentryOption;

#[derive(Debug, PartialEq)]
pub enum ClientRequest {
    Bye,
    Reset,
    End,
    Help,
    Quit,
    Option(PinentryOption),
    Cancel,
    Auth,
    Nop,

    GetPin,
    Confirm,
    Message,

    SetTimeout(i32),
    SetDescription(String),
    SetPrompt(String),
    SetTitle(String),
    SetOk(String),
    SetCancel(String),
    SetNotOk(String),
    SetError(String),
    SetRepeat,
    SetQualityBar,
    SetQualityBarTooltip(String),
    SetGenPin,
    SetGenPinTooltip(String),
    SetKeyInfo(String),
}

impl ClientRequest {
    pub fn parse(input: &str) -> ClientRequest {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "BYE" => ClientRequest::Bye,
            "RESET" => ClientRequest::Reset,
            "END" => ClientRequest::End,
            "HELP" => ClientRequest::Help,
            "QUIT" => ClientRequest::Quit,
            "CANCEL" => ClientRequest::Cancel,
            "AUTH" => ClientRequest::Auth,
            "NOP" => ClientRequest::Nop,
            "OPTION" => ClientRequest::Option(PinentryOption::parse(parts.get(1).unwrap_or(&""))),
            "GETPIN" => ClientRequest::GetPin,
            "CONFIRM" => ClientRequest::Confirm,
            "MESSAGE" => ClientRequest::Message,
            "SETTIMEOUT" => {
                let timeout = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
                ClientRequest::SetTimeout(timeout)
            }
            "SETDESC" => {
                let desc = parts[1..].join(" ");
                ClientRequest::SetDescription(desc)
            }
            "SETPROMPT" => {
                let prompt = parts[1..].join(" ");
                ClientRequest::SetPrompt(prompt)
            }
            "SETTITLE" => {
                let title = parts[1..].join(" ");
                ClientRequest::SetTitle(title)
            }
            "SETOK" => {
                let ok = parts[1..].join(" ");
                ClientRequest::SetOk(ok)
            }
            "SETCANCEL" => {
                let cancel = parts[1..].join(" ");
                ClientRequest::SetCancel(cancel)
            }
            "SETNOTOK" => {
                let notok = parts[1..].join(" ");
                ClientRequest::SetNotOk(notok)
            }
            "SETERROR" => {
                let error = parts[1..].join(" ");
                ClientRequest::SetError(error)
            }
            "SETREPEAT" => ClientRequest::SetRepeat,
            "SETQUALITYBAR" => ClientRequest::SetQualityBar,
            "SETQUALITYBARTOOLTIP" => {
                let tooltip = parts[1..].join(" ");
                ClientRequest::SetQualityBarTooltip(tooltip)
            }
            "SETGENPIN" => ClientRequest::SetGenPin,
            "SETGENPINTOOLTIP" => {
                let tooltip = parts[1..].join(" ");
                ClientRequest::SetGenPinTooltip(tooltip)
            }
            "SETKEYINFO" => {
                let keyinfo = parts[1..].join(" ");
                ClientRequest::SetKeyInfo(keyinfo)
            }
            _ => panic!("Invalid command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bye_command() {
        assert_eq!(ClientRequest::parse("BYE"), ClientRequest::Bye);
    }

    #[test]
    fn parses_reset_command() {
        assert_eq!(ClientRequest::parse("RESET"), ClientRequest::Reset);
    }

    #[test]
    fn parses_end_command() {
        assert_eq!(ClientRequest::parse("END"), ClientRequest::End);
    }

    #[test]
    fn parses_help_command() {
        assert_eq!(ClientRequest::parse("HELP"), ClientRequest::Help);
    }

    #[test]
    fn parses_quit_command() {
        assert_eq!(ClientRequest::parse("QUIT"), ClientRequest::Quit);
    }

    #[test]
    fn parses_cancel_command() {
        assert_eq!(ClientRequest::parse("CANCEL"), ClientRequest::Cancel);
    }

    #[test]
    fn parses_auth_command() {
        assert_eq!(ClientRequest::parse("AUTH"), ClientRequest::Auth);
    }

    #[test]
    fn parses_nop_command() {
        assert_eq!(ClientRequest::parse("NOP"), ClientRequest::Nop);
    }

    #[test]
    fn parses_option_command() {
        assert_eq!(
            ClientRequest::parse("OPTION formatted-passphrase"),
            ClientRequest::Option(PinentryOption::FormattedPassphrase)
        );

        assert_eq!(
            ClientRequest::parse("OPTION"),
            ClientRequest::Option(PinentryOption::UnknownOption)
        );
    }

    #[test]
    fn parses_getpin_command() {
        assert_eq!(ClientRequest::parse("GETPIN"), ClientRequest::GetPin);
    }

    #[test]
    fn parses_confirm_command() {
        assert_eq!(ClientRequest::parse("CONFIRM"), ClientRequest::Confirm);
    }

    #[test]
    fn parses_message_command() {
        assert_eq!(ClientRequest::parse("MESSAGE"), ClientRequest::Message);
    }

    #[test]
    fn parses_settimeout_command() {
        assert_eq!(
            ClientRequest::parse("SETTIMEOUT 10"),
            ClientRequest::SetTimeout(10)
        );
        assert_eq!(
            ClientRequest::parse("SETTIMEOUT"),
            ClientRequest::SetTimeout(0)
        );
    }

    #[test]
    fn parses_setdesc_command() {
        assert_eq!(
            ClientRequest::parse("SETDESC Enter your password"),
            ClientRequest::SetDescription("Enter your password".to_string())
        );
    }

    #[test]
    fn parses_setprompt_command() {
        assert_eq!(
            ClientRequest::parse("SETPROMPT Enter your password"),
            ClientRequest::SetPrompt("Enter your password".to_string())
        );
    }

    #[test]
    fn parses_settitle_command() {
        assert_eq!(
            ClientRequest::parse("SETTITLE Enter your password"),
            ClientRequest::SetTitle("Enter your password".to_string())
        );
    }

    #[test]
    fn parses_setok_command() {
        assert_eq!(
            ClientRequest::parse("SETOK OK"),
            ClientRequest::SetOk("OK".to_string())
        );
    }

    #[test]
    fn parses_setcancel_command() {
        assert_eq!(
            ClientRequest::parse("SETCANCEL Cancel"),
            ClientRequest::SetCancel("Cancel".to_string())
        );
    }

    #[test]
    fn parses_setnotok_command() {
        assert_eq!(
            ClientRequest::parse("SETNOTOK Not OK"),
            ClientRequest::SetNotOk("Not OK".to_string())
        );
    }

    #[test]
    fn parses_seterror_command() {
        assert_eq!(
            ClientRequest::parse("SETERROR Error"),
            ClientRequest::SetError("Error".to_string())
        );
    }

    #[test]
    fn parses_setrepeat_command() {
        assert_eq!(ClientRequest::parse("SETREPEAT"), ClientRequest::SetRepeat);
    }

    #[test]
    fn parses_setqualitybar_command() {
        assert_eq!(
            ClientRequest::parse("SETQUALITYBAR"),
            ClientRequest::SetQualityBar
        );
    }

    #[test]
    fn parses_setqualitybartooltip_command() {
        assert_eq!(
            ClientRequest::parse("SETQUALITYBARTOOLTIP Tooltip"),
            ClientRequest::SetQualityBarTooltip("Tooltip".to_string())
        );
    }

    #[test]
    fn parses_setgenpin_command() {
        assert_eq!(ClientRequest::parse("SETGENPIN"), ClientRequest::SetGenPin);
    }

    #[test]
    fn parses_setgenpintooltip_command() {
        assert_eq!(
            ClientRequest::parse("SETGENPINTOOLTIP Tooltip"),
            ClientRequest::SetGenPinTooltip("Tooltip".to_string())
        );
    }

    #[test]
    fn parses_setkeyinfo_command() {
        assert_eq!(
            ClientRequest::parse("SETKEYINFO Key info"),
            ClientRequest::SetKeyInfo("Key info".to_string())
        );
    }
}
