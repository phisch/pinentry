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
    pub fn parse(input: &str) -> Option<ClientRequest> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "BYE" => Some(ClientRequest::Bye),
            "RESET" => Some(ClientRequest::Reset),
            "END" => Some(ClientRequest::End),
            "HELP" => Some(ClientRequest::Help),
            "QUIT" => Some(ClientRequest::Quit),
            "CANCEL" => Some(ClientRequest::Cancel),
            "AUTH" => Some(ClientRequest::Auth),
            "NOP" => Some(ClientRequest::Nop),
            "OPTION" => Some(ClientRequest::Option(PinentryOption::parse(
                parts.get(1).unwrap_or(&""),
            ))),
            "GETPIN" => Some(ClientRequest::GetPin),
            "CONFIRM" => Some(ClientRequest::Confirm),
            "MESSAGE" => Some(ClientRequest::Message),
            "SETTIMEOUT" => {
                let timeout = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
                Some(ClientRequest::SetTimeout(timeout))
            }
            "SETDESC" => {
                let desc = parts[1..].join(" ");
                Some(ClientRequest::SetDescription(desc))
            }
            "SETPROMPT" => {
                let prompt = parts[1..].join(" ");
                Some(ClientRequest::SetPrompt(prompt))
            }
            "SETTITLE" => {
                let title = parts[1..].join(" ");
                Some(ClientRequest::SetTitle(title))
            }
            "SETOK" => {
                let ok = parts[1..].join(" ");
                Some(ClientRequest::SetOk(ok))
            }
            "SETCANCEL" => {
                let cancel = parts[1..].join(" ");
                Some(ClientRequest::SetCancel(cancel))
            }
            "SETNOTOK" => {
                let notok = parts[1..].join(" ");
                Some(ClientRequest::SetNotOk(notok))
            }
            "SETERROR" => {
                let error = parts[1..].join(" ");
                Some(ClientRequest::SetError(error))
            }
            "SETREPEAT" => Some(ClientRequest::SetRepeat),
            "SETQUALITYBAR" => Some(ClientRequest::SetQualityBar),
            "SETQUALITYBARTOOLTIP" => {
                let tooltip = parts[1..].join(" ");
                Some(ClientRequest::SetQualityBarTooltip(tooltip))
            }
            "SETGENPIN" => Some(ClientRequest::SetGenPin),
            "SETGENPINTOOLTIP" => {
                let tooltip = parts[1..].join(" ");
                Some(ClientRequest::SetGenPinTooltip(tooltip))
            }
            "SETKEYINFO" => {
                let keyinfo = parts[1..].join(" ");
                Some(ClientRequest::SetKeyInfo(keyinfo))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bye_command() {
        assert_eq!(ClientRequest::parse("BYE"), Some(ClientRequest::Bye));
    }

    #[test]
    fn parses_reset_command() {
        assert_eq!(ClientRequest::parse("RESET"), Some(ClientRequest::Reset));
    }

    #[test]
    fn parses_end_command() {
        assert_eq!(ClientRequest::parse("END"), Some(ClientRequest::End));
    }

    #[test]
    fn parses_help_command() {
        assert_eq!(ClientRequest::parse("HELP"), Some(ClientRequest::Help));
    }

    #[test]
    fn parses_quit_command() {
        assert_eq!(ClientRequest::parse("QUIT"), Some(ClientRequest::Quit));
    }

    #[test]
    fn parses_cancel_command() {
        assert_eq!(ClientRequest::parse("CANCEL"), Some(ClientRequest::Cancel));
    }

    #[test]
    fn parses_auth_command() {
        assert_eq!(ClientRequest::parse("AUTH"), Some(ClientRequest::Auth));
    }

    #[test]
    fn parses_nop_command() {
        assert_eq!(ClientRequest::parse("NOP"), Some(ClientRequest::Nop));
    }

    #[test]
    fn parses_option_command() {
        assert_eq!(
            ClientRequest::parse("OPTION formatted-passphrase"),
            Some(ClientRequest::Option(PinentryOption::FormattedPassphrase))
        );

        assert_eq!(
            ClientRequest::parse("OPTION"),
            Some(ClientRequest::Option(PinentryOption::UnknownOption))
        );
    }

    #[test]
    fn parses_getpin_command() {
        assert_eq!(ClientRequest::parse("GETPIN"), Some(ClientRequest::GetPin));
    }

    #[test]
    fn parses_confirm_command() {
        assert_eq!(
            ClientRequest::parse("CONFIRM"),
            Some(ClientRequest::Confirm)
        );
    }

    #[test]
    fn parses_message_command() {
        assert_eq!(
            ClientRequest::parse("MESSAGE"),
            Some(ClientRequest::Message)
        );
    }

    #[test]
    fn parses_settimeout_command() {
        assert_eq!(
            ClientRequest::parse("SETTIMEOUT 10"),
            Some(ClientRequest::SetTimeout(10))
        );
        assert_eq!(
            ClientRequest::parse("SETTIMEOUT"),
            Some(ClientRequest::SetTimeout(0))
        );
    }

    #[test]
    fn parses_setdesc_command() {
        assert_eq!(
            ClientRequest::parse("SETDESC Enter your password"),
            Some(ClientRequest::SetDescription(
                "Enter your password".to_string()
            ))
        );
    }

    #[test]
    fn parses_setprompt_command() {
        assert_eq!(
            ClientRequest::parse("SETPROMPT Enter your password"),
            Some(ClientRequest::SetPrompt("Enter your password".to_string()))
        );
    }

    #[test]
    fn parses_settitle_command() {
        assert_eq!(
            ClientRequest::parse("SETTITLE Enter your password"),
            Some(ClientRequest::SetTitle("Enter your password".to_string()))
        );
    }

    #[test]
    fn parses_setok_command() {
        assert_eq!(
            ClientRequest::parse("SETOK OK"),
            Some(ClientRequest::SetOk("OK".to_string()))
        );
    }

    #[test]
    fn parses_setcancel_command() {
        assert_eq!(
            ClientRequest::parse("SETCANCEL Cancel"),
            Some(ClientRequest::SetCancel("Cancel".to_string()))
        );
    }

    #[test]
    fn parses_setnotok_command() {
        assert_eq!(
            ClientRequest::parse("SETNOTOK Not OK"),
            Some(ClientRequest::SetNotOk("Not OK".to_string()))
        );
    }

    #[test]
    fn parses_seterror_command() {
        assert_eq!(
            ClientRequest::parse("SETERROR Error"),
            Some(ClientRequest::SetError("Error".to_string()))
        );
    }

    #[test]
    fn parses_setrepeat_command() {
        assert_eq!(
            ClientRequest::parse("SETREPEAT"),
            Some(ClientRequest::SetRepeat)
        );
    }

    #[test]
    fn parses_setqualitybar_command() {
        assert_eq!(
            ClientRequest::parse("SETQUALITYBAR"),
            Some(ClientRequest::SetQualityBar)
        );
    }

    #[test]
    fn parses_setqualitybartooltip_command() {
        assert_eq!(
            ClientRequest::parse("SETQUALITYBARTOOLTIP Tooltip"),
            Some(ClientRequest::SetQualityBarTooltip("Tooltip".to_string()))
        );
    }

    #[test]
    fn parses_setgenpin_command() {
        assert_eq!(
            ClientRequest::parse("SETGENPIN"),
            Some(ClientRequest::SetGenPin)
        );
    }

    #[test]
    fn parses_setgenpintooltip_command() {
        assert_eq!(
            ClientRequest::parse("SETGENPINTOOLTIP Tooltip"),
            Some(ClientRequest::SetGenPinTooltip("Tooltip".to_string()))
        );
    }

    #[test]
    fn parses_setkeyinfo_command() {
        assert_eq!(
            ClientRequest::parse("SETKEYINFO Key info"),
            Some(ClientRequest::SetKeyInfo("Key info".to_string()))
        );
    }
}
