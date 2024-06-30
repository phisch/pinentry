use std::{
    io::{BufRead, Write},
    vec,
};

use client_request::ClientRequest;
use error::AssuanError;
use response::Response;

pub mod client_request;
pub mod error;
pub mod option;
pub mod response;

pub struct Pinentry<R, W> {
    reader: R,
    writer: W,
    timeout: Option<i32>,
    description: Option<String>,
    prompt: Option<String>,
    title: Option<String>,
    ok_button: Option<String>,
    cancel_button: Option<String>,
    not_ok_button: Option<String>,
    error: Option<String>,
    repeat: bool,
    quality_bar: bool,
    quality_bar_tooltip: Option<String>,
    generate_pin: bool,
    generate_pin_tooltip: Option<String>,
    key_info: Option<String>,
    should_quit: bool,
}

impl<R, W> Pinentry<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Pinentry {
            reader,
            writer,
            timeout: None,
            description: None,
            prompt: None,
            title: None,
            ok_button: None,
            cancel_button: None,
            not_ok_button: None,
            error: None,
            repeat: false,
            quality_bar: false,
            quality_bar_tooltip: None,
            generate_pin: false,
            generate_pin_tooltip: None,
            key_info: None,
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        let greeting = Response::Ok(Some("Pleased to meet you".to_string())).to_string();
        writeln!(self.writer, "{}", greeting).unwrap();

        loop {
            let mut command = String::new();
            self.reader.read_line(&mut command).unwrap();
            command.pop();

            let responses = self.handle_request(ClientRequest::parse(&command));
            for response in responses {
                writeln!(self.writer, "{}", response).unwrap();
            }

            if self.should_quit {
                break;
            }
        }
    }

    fn handle_request(&mut self, request: Option<ClientRequest>) -> Vec<Response> {
        match request {
            Some(request) => match request {
                ClientRequest::Bye => {
                    self.should_quit = true;
                    vec![Response::Ok(Some("Closing connection".to_string()))]
                }
                ClientRequest::SetTimeout(timeout) => {
                    self.timeout = Some(timeout);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetDescription(desc) => {
                    self.description = Some(desc);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetPrompt(prompt) => {
                    self.prompt = Some(prompt);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetTitle(title) => {
                    self.title = Some(title);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetOk(ok_button) => {
                    self.ok_button = Some(ok_button);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetCancel(cancel_button) => {
                    self.cancel_button = Some(cancel_button);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetNotOk(not_ok_button) => {
                    self.not_ok_button = Some(not_ok_button);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetError(error) => {
                    self.error = Some(error);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetRepeat => {
                    self.repeat = true;
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetQualityBar => {
                    self.quality_bar = true;
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetQualityBarTooltip(tooltip) => {
                    self.quality_bar_tooltip = Some(tooltip);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetGenPin => {
                    self.generate_pin = true;
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetGenPinTooltip(tooltip) => {
                    self.generate_pin_tooltip = Some(tooltip);
                    vec![Response::Ok(None)]
                }
                ClientRequest::SetKeyInfo(key_info) => {
                    self.key_info = Some(key_info);
                    vec![Response::Ok(None)]
                }
                ClientRequest::Reset => {
                    self.timeout = None;
                    self.description = None;
                    self.prompt = None;
                    self.title = None;
                    self.ok_button = None;
                    self.cancel_button = None;
                    self.not_ok_button = None;
                    self.error = None;
                    self.repeat = false;
                    self.quality_bar = false;
                    self.quality_bar_tooltip = None;
                    self.generate_pin = false;
                    self.generate_pin_tooltip = None;
                    self.key_info = None;
                    vec![Response::Ok(None)]
                }
                _ => vec![Response::Error(AssuanError::NotImplemented)],
            },
            None => vec![Response::Error(AssuanError::UnknownIPCCommand)],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn assert_input_produces_output(
        input: Vec<&str>,
        output: Vec<&str>,
    ) -> Pinentry<Cursor<Vec<u8>>, Cursor<Vec<u8>>> {
        let input_buffer = Cursor::new((input.join("\n") + "\n").into_bytes());
        let output_buffer = Cursor::new(Vec::new());

        let mut pinentry = Pinentry::new(input_buffer, output_buffer);
        pinentry.run();

        assert_eq!(
            output.join("\n") + "\n",
            String::from_utf8(pinentry.writer.get_ref().to_vec()).unwrap()
        );

        pinentry
    }

    #[test]
    fn bye_command_closes_connection() {
        let pinentry = assert_input_produces_output(
            vec!["BYE"],
            vec!["OK Pleased to meet you", "OK Closing connection"],
        );
        assert_eq!(pinentry.should_quit, true);
    }

    #[test]
    fn setdesc_command_sets_description() {
        let pinentry = assert_input_produces_output(
            vec!["SETDESC Hello, world!", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.description, Some("Hello, world!".to_string()));
    }

    #[test]
    fn settimeout_command_sets_timeout() {
        let pinentry = assert_input_produces_output(
            vec!["SETTIMEOUT 10", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.timeout, Some(10));
    }

    #[test]
    fn setprompt_command_sets_prompt() {
        let pinentry = assert_input_produces_output(
            vec!["SETPROMPT Enter your password", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.prompt, Some("Enter your password".to_string()));
    }

    #[test]
    fn settitle_command_sets_title() {
        let pinentry = assert_input_produces_output(
            vec!["SETTITLE Enter your password", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.title, Some("Enter your password".to_string()));
    }

    #[test]
    fn setok_command_sets_ok_button() {
        let pinentry = assert_input_produces_output(
            vec!["SETOK OK", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.ok_button, Some("OK".to_string()));
    }

    #[test]
    fn setcancel_command_sets_cancel_button() {
        let pinentry = assert_input_produces_output(
            vec!["SETCANCEL Cancel", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.cancel_button, Some("Cancel".to_string()));
    }

    #[test]
    fn setnotok_command_sets_not_ok_button() {
        let pinentry = assert_input_produces_output(
            vec!["SETNOTOK Not OK", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.not_ok_button, Some("Not OK".to_string()));
    }

    #[test]
    fn seterror_command_sets_error() {
        let pinentry = assert_input_produces_output(
            vec!["SETERROR Error message", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.error, Some("Error message".to_string()));
    }

    #[test]
    fn setrepeat_command_sets_repeat() {
        let pinentry = assert_input_produces_output(
            vec!["SETREPEAT", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.repeat, true);
    }

    #[test]
    fn setqualitybar_command_sets_quality_bar() {
        let pinentry = assert_input_produces_output(
            vec!["SETQUALITYBAR", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.quality_bar, true);
    }

    #[test]
    fn setqualitybartooltip_command_sets_quality_bar_tooltip() {
        let pinentry = assert_input_produces_output(
            vec!["SETQUALITYBARTOOLTIP Tooltip", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.quality_bar_tooltip, Some("Tooltip".to_string()));
    }

    #[test]
    fn setgenpin_command_sets_generate_pin() {
        let pinentry = assert_input_produces_output(
            vec!["SETGENPIN", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.generate_pin, true);
    }

    #[test]
    fn setgenpintooltip_command_sets_generate_pin_tooltip() {
        let pinentry = assert_input_produces_output(
            vec!["SETGENPINTOOLTIP Tooltip", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.generate_pin_tooltip, Some("Tooltip".to_string()));
    }

    #[test]
    fn setkeyinfo_command_sets_key_info() {
        let pinentry = assert_input_produces_output(
            vec!["SETKEYINFO Key info", "BYE"],
            vec!["OK Pleased to meet you", "OK", "OK Closing connection"],
        );
        assert_eq!(pinentry.key_info, Some("Key info".to_string()));
    }

    #[test]
    fn reset_command_resets_all_fields() {
        let pinentry = assert_input_produces_output(
            vec![
                "SETDESC Hello, world!",
                "SETPROMPT Enter your password",
                "SETTITLE Enter your password",
                "SETOK OK",
                "SETCANCEL Cancel",
                "SETNOTOK Not OK",
                "SETERROR Error message",
                "SETREPEAT",
                "SETQUALITYBAR",
                "SETQUALITYBARTOOLTIP Tooltip",
                "SETGENPIN",
                "SETGENPINTOOLTIP Tooltip",
                "SETKEYINFO Key info",
                "RESET",
                "BYE",
            ],
            vec![
                "OK Pleased to meet you",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK",
                "OK Closing connection",
            ],
        );

        assert_eq!(pinentry.description, None);
        assert_eq!(pinentry.prompt, None);
        assert_eq!(pinentry.title, None);
        assert_eq!(pinentry.ok_button, None);
        assert_eq!(pinentry.cancel_button, None);
        assert_eq!(pinentry.not_ok_button, None);
        assert_eq!(pinentry.error, None);
        assert_eq!(pinentry.repeat, false);
        assert_eq!(pinentry.quality_bar, false);
        assert_eq!(pinentry.quality_bar_tooltip, None);
        assert_eq!(pinentry.generate_pin, false);
        assert_eq!(pinentry.generate_pin_tooltip, None);
        assert_eq!(pinentry.key_info, None);
    }

    #[test]
    fn unimplemented_command_returns_not_implemented_error() {
        assert_input_produces_output(
            vec!["CANCEL", "BYE"],
            vec![
                "OK Pleased to meet you",
                "ERR 536870981 Not implemented <User defined source 1>",
                "OK Closing connection",
            ],
        );
    }

    #[test]
    fn unknown_command_returns_unknown_ipc_command_error() {
        assert_input_produces_output(
            vec!["FOO", "BYE"],
            vec![
                "OK Pleased to meet you",
                "ERR 536871187 Unknown IPC command <User defined source 1>",
                "OK Closing connection",
            ],
        );
    }
}
